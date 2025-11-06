use {
    super::{
        get_theme,
        style::{Style, StyleItem},
    },
    htmlparser::{Token, Tokenizer},
    std::{collections::HashMap, sync::Once},
};

/// the wrapper element used to wrap the input string before parsing
const WRAPPER_ELEMENT: &str = "_wrapper_";

/// Represents the theme of the application.
/// A theme is a collection of styles, each style is associated with a tag.
/// For example, the tag `brand` might be associated with a style that has
/// a specific color and bold attribute.
///
/// To apply a theme to a string, we need to format such string as something
/// similar to HTML. For example, the string `This is a red text` might be
/// formatted as `<red>This is a red text</red>`.
///
/// We can also thin of theme tags as styling components.
pub struct Theme(pub HashMap<String, Style>);

#[derive(Clone, Debug)]
pub enum ParsedNode {
    Text(String),
    Tag(String, Vec<ParsedNode>),
}

impl Theme {
    /// creates a new theme from a hashmap
    pub fn new(map: HashMap<String, Vec<StyleItem>>) -> Theme {
        let mut theme = Theme(HashMap::new());

        for (key, value) in map {
            theme.insert(&key, Style(value));
        }

        theme
    }

    /// inserts a new style into the theme
    pub fn insert(&mut self, key: &str, style: Style) {
        self.0.insert(String::from(key), style);
    }

    /// applies the theme to the given node
    fn apply_node(&self, node: &ParsedNode, parent_style: &Style) -> String {
        let (node_style, child_style): (Style, Vec<String>) = match node {
            ParsedNode::Text(_) => (Style(vec![]), vec![]),
            ParsedNode::Tag(tag, children) => {
                let node_style = self.0.get(tag).cloned().unwrap_or_else(|| {
                    // Default style if the tag is not found in the theme
                    Style(vec![])
                });

                let child_style: Vec<String> =
                    children.iter().map(|child| self.apply_node(child, &node_style)).collect();

                (node_style, child_style)
            }
        };

        let combined_style = parent_style.merge(&node_style);

        match node {
            ParsedNode::Text(text) => combined_style.apply_to(text),
            ParsedNode::Tag(_, _) => {
                let styled_children: String = child_style.join("");
                combined_style.apply_to(&styled_children)
            }
        }
    }

    pub fn apply(&self, text: &str) -> String {
        let nodes = parse(text);

        nodes.iter().map(|node| self.apply_node(node, &Style(vec![]))).collect::<String>()
    }
}

/// parses a string into a collection of nodes
pub fn parse(string: &str) -> Vec<ParsedNode> {
    // wrap all the string in a wrapper element
    let string = &format!("<{}>{}</{}>", WRAPPER_ELEMENT, string, WRAPPER_ELEMENT);

    let mut nodes = vec![];
    let tokens = Tokenizer::from(&string[..]);
    let mut stack = vec![];

    for token in tokens {
        let token = match token {
            Ok(token) => token,
            Err(_) => continue,
        };

        match token {
            Token::ElementStart { prefix, local, .. } => {
                stack.push((prefix, local, vec![]));
            }
            Token::ElementEnd {
                end: htmlparser::ElementEnd::Close(_, local),
                ..
            } => {
                let (_, _, content) = stack.pop().unwrap();
                let parsed_node = ParsedNode::Tag(local.to_string(), content);
                if let Some(top) = stack.last_mut() {
                    top.2.push(parsed_node);
                } else {
                    nodes.push(parsed_node);
                }
            }
            Token::Text { text } => {
                if let Some(top) = stack.last_mut() {
                    top.2.push(ParsedNode::Text(text.to_string()));
                } else {
                    nodes.push(ParsedNode::Text(text.to_string()));
                }
            }
            _ => {}
        }
    }

    // filter out the wrapper element
    nodes
        .into_iter()
        .flat_map(|node| match node {
            ParsedNode::Tag(tag, content) if tag == WRAPPER_ELEMENT => content.clone(),
            _ => vec![node],
        })
        .collect::<Vec<_>>()
}

pub static INIT: Once = Once::new();
pub static mut GLOBAL_THEME: Option<Theme> = None;

pub fn initialize_theme() {
    INIT.call_once(|| unsafe {
        GLOBAL_THEME = Some(get_theme());
    });
}

/// prints a string by using the global theme
#[macro_export]
macro_rules! printth {
    // This pattern captures the arguments passed to the macro.
    ($($arg:tt)*) => {
        $crate::cli::color::theme::initialize_theme();
        unsafe {
            let theme_ptr = std::ptr::addr_of!($crate::cli::color::theme::GLOBAL_THEME);
            if let Some(theme) = (*theme_ptr).as_ref() {
                let formatted_string = format!($($arg)*);
                let themed_string = theme.apply(&formatted_string);
                println!("{}", themed_string);
            }
        }
    };
}
