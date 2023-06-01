/// Enumeration of possible display errors
#[derive(Debug, Clone, Copy)]
pub enum DisplayError {

    /// Happens when a display manager is not supported.
    NotSupported,

    /// Happens when no display server is found.
    NoDisplayServer,

    /// Happens when trying to resize a [Window](super::desktop::window::Window) outside of allowed boundaries.
    SizeError,

    /// Happens when trying get hardware screen details failed.
    ScreenDetailError,

}