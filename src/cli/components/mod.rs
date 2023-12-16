mod help;
mod logo;

pub use help::Help;
pub use logo::Logo;

pub trait Component {
    /// render method
    fn render(&self) -> String;
}
