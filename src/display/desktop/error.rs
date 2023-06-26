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

    /// Happens when trying to add a subwindow to a parent locked by a modal window.
    ParentIsLockedByModal,

    /// Happens when trying to open a modal window without parent
    ModalRequiresParent,

    /// Happens when trying to make a parent window a sub window.
    SubHasChild,

    /// Happens when trying to show a window that is already showed
    WindowAlreadyOpened,

    /// Happens when trying to close a window that was not opened.
    WindowNotOpened,

    /// Happens when trying to show a subwindow on a parent that is not opened.
    ParentNotOpened

}