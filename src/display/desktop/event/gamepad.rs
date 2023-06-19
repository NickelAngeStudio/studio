
/// Enumeration of possible gamepad events
#[derive(Debug, Copy, Clone)]
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