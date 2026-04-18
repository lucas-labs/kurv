mod help;
mod logo;

pub use {help::Help, logo::Logo};

pub trait Component {
    /// render method
    fn render(&self) -> String;
}
