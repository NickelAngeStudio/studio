//! Desktop components abstraction of display.
//! 
//! Contains components needed to create and manage a desktop windows.

// Enumeration of desktop error
pub mod error;

// Hardware screen list
pub mod screen;

// Window events
pub mod event;

// Window managers
pub mod manager;

// Window
#[doc(hidden)]
pub mod window;

pub use window::Window as Window;

// Properties
pub mod property;

// Contains Desktop tests
#[cfg(test)]
#[doc(hidden)]
pub mod tests;