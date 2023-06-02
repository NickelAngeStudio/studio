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
    pub is_visible : bool,

    /// Indicate if cursor is confined to the window boundaries or not.
    pub is_confined : bool, 

    /// Index of the left button
    pub left_button : u8,

    /// Index of the right button
    pub right_button : u8,

    /// Index of the middle button
    pub middle_button : u8,

    /// Index of the next button
    pub next_button: u8,

    /// Index of the previous button
    pub previous_button : u8,

    /// Index of the scroll up button
    pub scroll_up : u8,

    /// Index of the scroll down button
    pub scroll_down : u8,

    /// Index of the scroll left button
    pub scroll_left : u8,

    /// Index of the scroll right button
    pub scroll_right : u8,



}


impl PointerProperty {
    /// Create a new [PointerProperty] with default values.
    pub fn new(left_button : u8, right_button : u8, middle_button : u8, next_button: u8, previous_button : u8,
        scroll_up: u8, scroll_down: u8, scroll_left:u8, scroll_right:u8) -> PointerProperty {
        PointerProperty{ 
            mode: PointerMode::Pointer, 
            position: (0,0), 
            is_visible: true, 
            is_confined: false,
            left_button,
            right_button,
            middle_button,
            next_button,
            previous_button,
            scroll_up,
            scroll_down,
            scroll_left,
            scroll_right,
        }
    }

}