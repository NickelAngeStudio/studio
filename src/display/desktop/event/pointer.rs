
/// Enumeration of possible pointer buttons
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PointerButton {
    /// Left button of a pointer device.
    LeftButton,

    /// Right button of a pointer device.
    RightButton,

    /// Middle button of a pointer device.
    MiddleButton,

    /// Previous button of a pointer device.
    PreviousButton,

    /// Next button of a pointer device
    NextButton,

    /// Scroll up button of pointer device.
    ScrollUp,

    /// Scroll down button of pointer device.
    ScrollDown,

    /// Scroll left button of pointer device.
    ScrollLeft,

    /// Scroll right button of pointer device.
    ScrollRight,

    /// Other pointer button.
    Other(u16)
}


/// Enumeration of possible pointer events
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EventPointer {

    /// Pointer move event. Provides new (x, y) position. Only when in pointer mode.
    Moved((i32, i32)),

    /// Pointer acceleration event.  Provides delta (x, y). Only when in acceleration mode.
    Acceleration((i32, i32)),

    /// Pointer button down event. Provides button number (up to 255) and cursor position (x,y).
    ButtonDown(PointerButton, (i32, i32)),

    /// Pointer button up event. Provides button number (up to 255) and cursor position (x,y).
    ButtonUp(PointerButton, (i32, i32)),
}