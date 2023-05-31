/// Enumeration of possible [KWindow] cursor mode used for [DisplayEvent].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CursorMode {
    /// [DisplayEventMouse] events will give the (x,y) location of the cursor on the window. 
    /// 
    /// Usually used for user interfaces interactions.
    Pointer,

    /// [DisplayEventMouse] events will give the (x,y) acceleration of the cursor instead of the position.
    /// 
    /// Usually used for 3d camera and direct mouse inputs.
    Acceleration,
}

/// [KWindow] cursor properties.
pub struct CursorProperty {
    /// Motion mode of the mouse
    pub mode : CursorMode,

    /// Current cursor position
    pub position : (i32, i32),

    /// Is cursor visible?
    pub visible : bool,

    /// Is cursor confined?
    pub confined : bool, 
}


impl CursorProperty {
    /// Create a new [KCursorProperty] with default values.
    pub fn new() -> CursorProperty {
        CursorProperty{ 
            mode: CursorMode::Pointer, 
            position: (0,0), 
            visible: true, 
            confined: false
        }
    }

}