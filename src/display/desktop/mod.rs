//! Desktop components abstraction of display.
//! 
//! Contains components needed to create and manage a desktop windows.

// Enumeration of desktop error
pub mod error;

// Hardware screen list
pub mod screen;

// Window manager definition.
//pub mod manager;

// Window definition.
pub mod window;

// Window properties.
pub mod property;

// Window events input such as mouse, keyboard, etc..
pub mod event;

// Window providers
pub mod provider;