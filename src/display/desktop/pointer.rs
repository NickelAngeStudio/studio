//! Pointer properties and modes.


/// Enumeration of possible [Window](super::window::Window) pointer mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PointerMode {
    /// [EventMouse](super::event::EventMouse) events will give the (x,y) location of the cursor on the window. 
    /// 
    /// Usually used for user interfaces interactions.
    Pointer,

    /// [EventMouse](super::event::EventMouse) events will give the (x,y) acceleration of the cursor instead of the position.
    /// 
    /// Usually used for 3d camera and direct mouse inputs.
    Acceleration,
}

/// [Window](super::window::Window) cursor properties such as mode, position, etc.
pub struct PointerProperty {
    /// [PointerMode] used for [EventMouse](super::event::EventMouse) events.
    pub mode : PointerMode,

    /// Current cursor position on the window.
    pub position : (i32, i32),

    /// Indicate if cursor is visible or hidden.
    pub visible : bool,

    /// Indicate if cursor is confined to the window boundaries or not.
    pub confined : bool, 
}


impl PointerProperty {
    /// Create a new [PointerProperty] with default values.
    pub fn new() -> PointerProperty {
        PointerProperty{ 
            mode: PointerMode::Pointer, 
            position: (0,0), 
            visible: true, 
            confined: false
        }
    }

}