mod style;
pub mod theme;

pub use theme::Theme;

use {
    self::style::StyleItem,
    crossterm::style::{Attribute, Color},
    velcro::hash_map_from,
};

/// 🎨 » returns the theme of the application
pub fn get_theme() -> Theme {
    Theme::new(hash_map_from! {
        "head": [
            StyleItem::Color(Color::DarkBlue),
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
        "blue": [
            StyleItem::Color(Color::DarkBlue),
        ],
        "yellow": [
            StyleItem::Color(Color::Yellow),
        ],
        "error": [
            StyleItem::Color(Color::Red),
            StyleItem::Attr(Attribute::Bold),
        ],
        "b": [
            StyleItem::Attr(Attribute::Bold),
        ],
        "error": [
            StyleItem::Color(Color::Red),
        ],
        "warn": [
            StyleItem::Color(Color::Yellow),
        ],
        "info": [
            StyleItem::Color(Color::White),
        ],
        "debug": [
            StyleItem::Color(Color::Magenta),
        ],
        "trace": [
            StyleItem::Color(Color::Cyan),
        ],
    })
}
