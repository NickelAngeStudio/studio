/// # Re-export for Public API
#[doc(inline)]
pub use mouse::WindowEventMouse as WindowEventMouse;
pub use window::WindowEventWindow as WindowEventWindow;
pub use controller::WindowEventController as WindowEventController;
pub use keyboard::WindowEventKeyboard as WindowEventKeyboard;

// Kleio window events
#[doc(hidden)]
pub mod window;

// Kleio keyboard events
#[doc(hidden)]
pub mod keyboard;

// Kleio mouse events
#[doc(hidden)]
pub mod mouse;

// Kleio controller events
#[doc(hidden)]
pub mod controller;

// Kleio events dispatcher and receiver
#[doc(hidden)]
pub mod dispatcher;

/// Union of possible events into an enumeration.
#[derive(Copy, Clone)]
pub enum WindowEvent {

    /// No event.
    None,

    /// Window events
    Window(WindowEventWindow),

    /// Keyboard events
    Keyboard(WindowEventKeyboard),

    /// Mouse events
    Mouse(WindowEventMouse),

    /// Controller events
    Controller(WindowEventController),

    /// Unknown/Unhandled by Kleio event
    Unknown,
}

impl std::fmt::Debug for WindowEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Window(arg0) => f.debug_tuple("Window").field(arg0).finish(),
            Self::Keyboard(arg0) => f.debug_tuple("Keyboard").field(arg0).finish(),
            Self::Mouse(arg0) => f.debug_tuple("Mouse").field(arg0).finish(),
            Self::Controller(arg0) => f.debug_tuple("Controller").field(arg0).finish(),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
