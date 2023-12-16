mod logo;
mod help;

pub use logo::Logo;
pub use help::Help;

pub trait Component  {
    /// render method
    fn render(&self) -> String;
}


// use std::fmt::Display;
// implement Display for all Components
// impl Display for dyn Component {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.render())
//     }
// }

// The issue might be due to the fact that you're trying to implement Display for a trait object (dyn Component), not for the Logo struct itself. In Rust, trait objects are a way to call methods on objects that implement a certain trait, without knowing their concrete type.

// However, when you try to use a trait object (like dyn Component) as if it were a concrete type (like Logo), you can run into issues. This is because trait objects don't automatically inherit the implementations of other traits like Display.

// To fix this, you could either:

// Implement Display directly for Logo, or
// Create a wrapper struct that holds a Box<dyn Component>, and implement Display for that wrapper struct.
// Here's an example of the second approach:

// ```rust
// pub struct ComponentWrapper(Box<dyn Component>);

// impl Display for ComponentWrapper {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0.render())
//     }
// }
// ```

// Then, when you want to print a Logo, you would wrap it in a ComponentWrapper:

// ```rust
// let logo = Logo::new();
// let wrapper = ComponentWrapper(Box::new(logo));
// println!("{}", wrapper);
// ```

// This way, you're implementing Display for a concrete type (ComponentWrapper), not a trait object.
