//! Window events input such as mouse, keyboard, etc..

use self::{window::EventWindow, keyboard::EventKeyboard, pointer::EventPointer, gamepad::EventGamepad, sub::EventSubWindow};

pub mod window;

pub mod keyboard;

pub mod pointer;

pub mod gamepad;

pub mod sub;

/// Union of possible events into an enumeration.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Event {

    /// No event.
    None,

    /// Window events
    Window(EventWindow),

    /// SubWindow events
    SubWindow(EventSubWindow),

    /// Keyboard events
    Keyboard(EventKeyboard),

    /// Pointer events
    Pointer(EventPointer),

    /// Gamepad events
    Gamepad(EventGamepad),
}



