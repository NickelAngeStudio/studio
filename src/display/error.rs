/// Enumeration of possible display errors
#[derive(Debug, Clone, Copy)]
pub enum DisplayError {

    /// Happens when a display manager is not supported.
    NotSupported,

    /// Happens when no display server is found.
    NoDisplayServer,

    /// Happens when trying to resize a [Window](super::desktop::window::Window) outside of allowed boundaries.
    SizeError,

    /// Happens when trying to set a read-only [WindowProperty](super::desktop::property::WindowProperty).
    ReadOnlyProperty,

    /// Happens when trying get hardware screen details failed.
    ScreenDetailError,

    /// Happens when a child is already owned by another window.
    ChildAlreadyOwned,

    /// Happens when a child window is not found.
    ChildNotFound,

}