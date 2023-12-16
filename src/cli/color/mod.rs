mod style;
pub mod theme;
use crossterm::style::{Color, Attribute};
pub use theme::Theme;
use {
    self::style::StyleItem,
    velcro::hash_map_from,
};

/// 🎨 ⇝ returns the theme of the application
pub fn get_theme() -> Theme {
    Theme::new(hash_map_from! {
        "head": [
            StyleItem::Color(Color::Blue),
            StyleItem::Attr(Attribute::Bold),
        ],
        "highlight": [
            StyleItem::Color(Color::White),
            StyleItem::Attr(Attribute::Bold),
        ],
        "dim": [
            StyleItem::Attr(Attribute::Dim),
        ],
        "magenta": [
            StyleItem::Color(Color::Magenta),
        ],
        "white": [
            StyleItem::Color(Color::White),
        ],
        "green": [
            StyleItem::Color(Color::Green),
        ],
        "error": [
            StyleItem::Color(Color::Red),
            StyleItem::Attr(Attribute::Bold),
        ],
    })
}
