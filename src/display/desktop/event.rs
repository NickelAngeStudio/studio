//! Window events input such as mouse, keyboard, etc..

/// Union of possible events into an enumeration.
#[derive(Copy, Clone)]
pub enum Event {

    /// No event.
    None,

    /// Window events
    Window(EventWindow),

    /// Keyboard events
    Keyboard(EventKeyboard),

    /// Mouse events
    Mouse(EventMouse),

    /// Controller events
    Controller(EventGamepad),
}

impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Window(arg0) => f.debug_tuple("Window").field(arg0).finish(),
            Self::Keyboard(arg0) => f.debug_tuple("Keyboard").field(arg0).finish(),
            Self::Mouse(arg0) => f.debug_tuple("Mouse").field(arg0).finish(),
            Self::Controller(arg0) => f.debug_tuple("Controller").field(arg0).finish(),
        }
    }
}

/// Enumeration of possible events for a window
#[derive(Copy, Clone)]
pub enum EventWindow {

    /// Happens when Display is shown.
    Shown(),

    /// Happens when Display is hidden.
    Hidden(),

    /// Happens when Display is exposed/damaged, meaning part of drawing is lost and need to be redraw.
    /// Provides position (x, y) and size (width, height) of region exposed. 
    Exposed((i32, i32), (u32, u32)),

    /// Happens when Display is moved. Provides (x,y) of new position.
    Moved((i32, i32)),

    /// Happens when Display is moved and resized. Provides (x,y) of new position and (height, width) of new size.
    MovedResized((i32, i32), (u32, u32)),

    /// Happens when Display is Resized. Provides (height, width) of new size.
    Resized((u32, u32)),

    /// Happens when Display is minimized.
    /// 
    /// # Known issue(s)
    /// * `(Linux only)` Won't trigger if window is maximized.
    Minimized(),

    /// Happens when Display is maximized.
    Maximized(),

    /// Happens when Display is set fullscreen.
    Fullscreen(),

    /// Happens when Display is restored from minimized, maximized or fullscreen.
    Restored(),

    /// Happens when cursor enter Display.
    CursorEnter(),

    /// Happens when cursor leave Display.
    CursorLeave(),

    /// Happens when Display gain focus.
    Focus(),

    /// Happens when Display lose focus.
    Blur(),

    /// Happens when a child closed.
    ChildClosed(),

    /// Happens when a close request is sent from the client.
    CloseRequest(),

    /// Happens when a window was closed.
    Closed(),
}

impl std::fmt::Debug for EventWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shown() => f.debug_tuple("Shown").finish(),
            Self::Hidden() => f.debug_tuple("Hidden").finish(),
            Self::Exposed(arg0, arg1) => f.debug_tuple("Exposed").field(arg0).field(arg1).finish(),
            Self::Moved(arg0) => f.debug_tuple("Moved").field(arg0).finish(),
            Self::MovedResized(arg0, arg1) => f.debug_tuple("MovedResized").field(arg0).field(arg1).finish(),
            Self::Resized(arg0) => f.debug_tuple("Resized").field(arg0).finish(),
            Self::Minimized() => f.debug_tuple("Minimized").finish(),
            Self::Maximized() => f.debug_tuple("Maximized").finish(),
            Self::Fullscreen() => f.debug_tuple("Fullscreen").finish(),
            Self::Restored() => f.debug_tuple("Restored").finish(),
            Self::CursorEnter() => f.debug_tuple("CursorEnter").finish(),
            Self::CursorLeave() => f.debug_tuple("CursorLeave").finish(),
            Self::Focus() => f.debug_tuple("Focus").finish(),
            Self::Blur() => f.debug_tuple("Blur").finish(),
            Self::ChildClosed() => f.debug_tuple("ChildClosed").finish(),
            Self::CloseRequest() => f.debug_tuple("CloseRequest").finish(),
            Self::Closed() => f.debug_tuple("Closed").finish(),
        }
    }
}


/// Enumeration of possible Keyboard events
#[derive(Copy, Clone)]
pub enum EventKeyboard {

    // Keyboard key down event. Provides unicode of key pressed.
    KeyDown(u32),

    // Keyboard key up event. Provides unicode of key released.
    KeyUp(u32),
}

impl std::fmt::Debug for EventKeyboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeyDown(arg0) => f.debug_tuple("KeyDown").field(arg0).finish(),
            Self::KeyUp(arg0) => f.debug_tuple("KeyUp").field(arg0).finish(),
        }
    }
}

/// Enumeration of possible mouse events
#[derive(Copy, Clone)]
pub enum EventMouse {

    // Mouse move event. Provides new (x, y) position. Only when in pointer mode.
    Moved((i32, i32)),

    // Mouse acceleration event.  Provides delta (x, y). Only when in acceleration mode.
    Acceleration((i32, i32)),

    // Mouse button down event. Provides button number (up to 255) and cursor position (x,y).
    ButtonDown(u8, (i32, i32)),

    // Mouse button up event. Provides button number (up to 255) and cursor position (x,y).
    ButtonUp(u8, (i32, i32)),

    // Mouse wheel event. Provide amount scrolled horizontally and vertically.
    Wheel(i32, i32),

}

impl std::fmt::Debug for EventMouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Moved(arg0) => f.debug_tuple("Moved").field(arg0).finish(),
            Self::Acceleration(arg0) => f.debug_tuple("Acceleration").field(arg0).finish(),
            Self::ButtonDown(arg0, arg1) => f.debug_tuple("ButtonDown").field(arg0).field(arg1).finish(),
            Self::ButtonUp(arg0, arg1) => f.debug_tuple("ButtonUp").field(arg0).field(arg1).finish(),
            Self::Wheel(arg0, arg1) => f.debug_tuple("Wheel").field(arg0).field(arg1).finish(),
        }
    }
}

/// Enumeration of possible gamepad events
#[derive(Copy, Clone)]
pub enum EventGamepad {

    /// Happens when a controller device has been connected. Provides controller id.
    Connected(u8),

    /// Happens when a controller device has been disconnected. Provides controller id.
    Disconnected(u8),

    /// Happens when a controller button is pressed. Provides controller id and button id.
    ButtonDown(u8, u8),

    /// Happens when a controller button is released. Provides controller id and button id.
    ButtonUp(u8, u8),

    /// Happens when a controller axis is used. Provides controller id, axis id and axis value range from (range: -32768 to 32767).
    /// 
    /// # Reference(s)
    /// Based on SDL_ControllerAxisEvent : <https://wiki.libsdl.org/SDL2/SDL_ControllerAxisEvent>
    Axis(u8, u8, i16)

}

impl std::fmt::Debug for EventGamepad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Connected(arg0) => f.debug_tuple("Connected").field(arg0).finish(),
            Self::Disconnected(arg0) => f.debug_tuple("Disconnected").field(arg0).finish(),
            Self::ButtonDown(arg0, arg1) => f.debug_tuple("ButtonDown").field(arg0).field(arg1).finish(),
            Self::ButtonUp(arg0, arg1) => f.debug_tuple("ButtonUp").field(arg0).field(arg1).finish(),
            Self::Axis(arg0, arg1, arg2) => f.debug_tuple("Axis").field(arg0).field(arg1).field(arg2).finish(),
        }
    }
}
