use {
    crossterm::style::Attribute,
    kurv::cli::color::{
        StyleItem,
        theme::{ParsedNode, Theme, parse},
    },
    std::collections::HashMap,
};

#[test]
fn test_parse_simple_text() {
    let nodes = parse("hello world");
    assert_eq!(nodes.len(), 1);

    match &nodes[0] {
        ParsedNode::Text(text) => assert_eq!(text, "hello world"),
        _ => panic!("Expected Text node"),
    }
}

#[test]
fn test_parse_with_single_tag() {
    let nodes = parse("<error>Error occurred</error>");
    assert_eq!(nodes.len(), 1);

    match &nodes[0] {
        ParsedNode::Tag(tag, children) => {
            assert_eq!(tag, "error");
            assert_eq!(children.len(), 1);
            match &children[0] {
                ParsedNode::Text(text) => assert_eq!(text, "Error occurred"),
                _ => panic!("Expected Text node inside tag"),
            }
        }
        _ => panic!("Expected Tag node"),
    }
}

#[test]
fn test_parse_mixed_text_and_tags() {
    let nodes = parse("Status: <green>OK</green> and running");
    assert_eq!(nodes.len(), 3);

    match &nodes[0] {
        ParsedNode::Text(text) => assert_eq!(text, "Status: "),
        _ => panic!("Expected Text node"),
    }

    match &nodes[1] {
        ParsedNode::Tag(tag, children) => {
            assert_eq!(tag, "green");
            assert_eq!(children.len(), 1);
        }
        _ => panic!("Expected Tag node"),
    }

    match &nodes[2] {
        ParsedNode::Text(text) => assert_eq!(text, " and running"),
        _ => panic!("Expected Text node"),
    }
}

#[test]
fn test_parse_nested_tags() {
    let nodes = parse("<white><b>Bold White</b></white>");
    assert_eq!(nodes.len(), 1);

    match &nodes[0] {
        ParsedNode::Tag(tag, children) => {
            assert_eq!(tag, "white");
            assert_eq!(children.len(), 1);

            match &children[0] {
                ParsedNode::Tag(inner_tag, inner_children) => {
                    assert_eq!(inner_tag, "b");
                    assert_eq!(inner_children.len(), 1);
                    match &inner_children[0] {
                        ParsedNode::Text(text) => assert_eq!(text, "Bold White"),
                        _ => panic!("Expected Text in nested tag"),
                    }
                }
                _ => panic!("Expected nested Tag"),
            }
        }
        _ => panic!("Expected Tag node"),
    }
}

#[test]
fn test_parse_multiple_tags() {
    let nodes = parse("<error>Error</error> <warn>Warning</warn>");
    assert_eq!(nodes.len(), 3); // error tag, space text, warn tag

    match &nodes[0] {
        ParsedNode::Tag(tag, _) => assert_eq!(tag, "error"),
        _ => panic!("Expected Tag node"),
    }

    match &nodes[1] {
        ParsedNode::Text(text) => assert_eq!(text, " "),
        _ => panic!("Expected Text node"),
    }

    match &nodes[2] {
        ParsedNode::Tag(tag, _) => assert_eq!(tag, "warn"),
        _ => panic!("Expected Tag node"),
    }
}

#[test]
fn test_theme_creation() {
    let mut style_map = HashMap::new();
    style_map.insert("test".to_string(), vec![StyleItem::Attr(Attribute::Bold)]);

    let theme = Theme::new(style_map);

    assert!(theme.0.contains_key("test"));
}

#[test]
fn test_theme_apply_simple_text() {
    let theme = Theme::new(HashMap::new());
    let result = theme.apply("hello");

    // without any styles, should return the text as-is (possibly with ANSI resets)
    assert!(result.contains("hello"));
}

#[test]
fn test_theme_apply_with_style() {
    let mut style_map = HashMap::new();
    style_map.insert("test".to_string(), vec![StyleItem::Attr(Attribute::Bold)]);

    let theme = Theme::new(style_map);
    let result = theme.apply("<test>Bold Text</test>");

    // should contain the text
    assert!(result.contains("Bold Text"));

    // should contain ANSI codes (basic check)
    assert!(result.len() > "Bold Text".len());
}

#[test]
fn test_parse_empty_string() {
    let nodes = parse("");
    // empty string might result in empty vec or single empty text node
    // depending on parser implementation
    assert!(
        nodes.is_empty()
            || (nodes.len() == 1 && matches!(&nodes[0], ParsedNode::Text(t) if t.is_empty()))
    );
}

#[test]
fn test_theme_apply_unknown_tag() {
    let theme = Theme::new(HashMap::new());
    let result = theme.apply("<unknowntag>Text</unknowntag>");

    // unknown tags should still pass through the text
    assert!(result.contains("Text"));
}

#[test]
fn test_parse_with_special_characters() {
    let nodes = parse("Text with <tag>special & chars</tag>");

    // should handle special characters in text
    let has_ampersand = nodes.iter().any(|node| match node {
        ParsedNode::Tag(_, children) => {
            children.iter().any(|child| matches!(child, ParsedNode::Text(t) if t.contains("&")))
        }
        ParsedNode::Text(t) => t.contains("&"),
    });

    assert!(has_ampersand);
}
