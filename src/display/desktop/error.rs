//! Enumeration of possible desktop errors.

/// Enumeration of possible desktop errors
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

    /// Happens when trying to make a window it's own parent.
    ParentSameAsSub,

    /// Happens when trying to make a target child window it's parent.
    ParentIsParent,

    /// Happens when trying to show a window with a position relative to a parent but no parent is specified.
    PositionNoParent,

    /// Happens when trying to change a window parent that is locked.
    ParentIsLocked,

}