use super::{Component, Logo};

static SUMMARY: &str = "{summary}";
static OPTIONS_HEAD: &str = "<head>options</head>";
static SUBCOMMANDS_HEAD: &str = "<head>commands</head>";

type Item<'a> = &'a str;
type Aliases<'a> = Vec<&'a str>;
type Desc<'a> = &'a str;
type AliasedItem<'a> = Vec<(Item<'a>, Aliases<'a>, Desc<'a>)>;

pub struct Help<'a> {
    pub command: &'a str,
    pub error: Option<&'a str>,
    pub summary: Option<&'a str>,
    pub options: Option<AliasedItem<'a>>,
    pub subcommands: Option<AliasedItem<'a>>,
}

impl<'a> Component for Help<'a> {
    fn render(&self) -> String {
        let logo = Logo {};

        let mut help = String::new();

        help.push_str(&logo.render());

        if let Some(error) = &self.error {
            help.push_str(&format!("\n😱 <error>{error}</error>\n"));
        }

        // Modify the usage string based on the presence of options and subcommands
        let mut usage = String::from("\n<head>usage</head>\n  <highlight>{command}</highlight>");
        if self.options.is_some() {
            usage.push_str(" <dim>[options]</dim>");
        }
        if self.subcommands.is_some() {
            usage.push_str(" <dim>[command]</dim>");
        }

        usage.push_str(" <dim>[args...]</dim>");

        help.push_str(&usage.replace("{command}", &self.command));

        if let Some(summary) = &self.summary {
            help.push_str(&format!("\n\n{}", SUMMARY.replace("{summary}", summary)));
        }

        if let Some(options) = &self.options {
            help.push_str("\n\n");
            help.push_str(OPTIONS_HEAD);
            help.push_str(&self.render_items(options, ", "));
        }

        if let Some(subcommands) = &self.subcommands {
            help.push_str("\n\n");
            help.push_str(SUBCOMMANDS_HEAD);
            help.push_str(&self.render_items(subcommands, "|"));
        }

        help.push_str("\n");

        help
    }
}

impl<'a> Help<'a> {
    fn render_items(&self, items: &AliasedItem<'a>, separator: &str) -> String {
        // Calculate the gutter space dynamically based on the length of the longest item
        let gutter_space = items
            .iter()
            .map(|(item, _, _)| item.len())
            .max()
            .unwrap_or(0)
            + 4;

        items
            .iter()
            .map(|(item, aliases, description)| {
                let item = match aliases.len() {
                    0 => item.to_string(),
                    _ => format!("{}{separator}{}", item, aliases.join(separator)),
                };

                format!(
                    "\n  <highlight>{:<width$}</highlight>{}",
                    item,
                    description,
                    width = gutter_space
                )
            })
            .collect()
    }
}
