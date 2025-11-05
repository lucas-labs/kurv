use crossterm::style::{Attribute, Color, Stylize, style};

/// converts a hex color to a crossterm `Color::Rgb`
fn hex_to_rgb(hex: &str) -> Result<Color, std::num::ParseIntError> {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;

    Ok(Color::Rgb { r, g, b })
}

/// a single style item
#[derive(Clone)]
#[allow(dead_code)]
pub enum StyleItem {
    Attr(Attribute),
    Rgb(&'static str),
    Color(Color),
}

/// a collection of style items
#[derive(Clone)]
pub struct Style(pub Vec<StyleItem>);

impl Style {
    /// merges two styles into one
    pub fn merge(&self, other: &Style) -> Style {
        let merged_items: Vec<StyleItem> =
            self.0.iter().cloned().chain(other.0.iter().cloned()).collect();
        Style(merged_items)
    }

    /// applies the style to the given text
    pub fn apply_to(&self, text: &str) -> String {
        let mut styled_text = text.to_owned();

        for style_item in &self.0 {
            match style_item {
                StyleItem::Attr(attribute) => {
                    styled_text = style(styled_text).attribute(*attribute).to_string();
                }
                StyleItem::Rgb(hex_color) => {
                    if let Ok(rgb_color) = hex_to_rgb(hex_color) {
                        styled_text = style(styled_text).with(rgb_color).to_string();
                    }
                }
                StyleItem::Color(color) => {
                    styled_text = style(styled_text).with(*color).to_string();
                }
            }
        }

        styled_text
    }
}
