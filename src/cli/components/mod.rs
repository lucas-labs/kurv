mod logo;
mod help;

pub use logo::Logo;
pub use help::Help;

pub trait Component  {
    /// render method
    fn render(&self) -> String;
}
