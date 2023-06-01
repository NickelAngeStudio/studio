/// Contains X11 C Bind
pub mod cbind;

/// Contains X11 screen fetch function
pub mod screen;

/// Contains X11 atoms
pub mod atom;

/// Contains X11 Window implementation
pub mod window;
pub use window::X11Window as X11Window;

/// Contains X11 Window events bind
pub(crate) mod event;