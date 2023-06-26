//! Subwindow events

/// Enumeration of possible subwindow events given to parent.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EventSubWindow {

    /// Happens when a subwindow is added to parent.
    SubAdded,

    /// Happens when a subwindow is removed from parent.
    SubRemoved,

    /// Happens when a modal subwindow is opened.
    SubModalOpened,

    /// Happens when a modal subwindow is closed.
    SubModalClosed,
}