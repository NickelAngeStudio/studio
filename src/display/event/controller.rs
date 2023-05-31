/// Enumeration of possible controller events
#[derive(Copy, Clone)]
pub enum WindowEventController {

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

impl std::fmt::Debug for WindowEventController {
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
