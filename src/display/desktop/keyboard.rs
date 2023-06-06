//! Keyboard properties


/// Contains keyboard properties.
pub struct KeyboardProperty {
    /// If true, key will be repeated when pressed down.
    pub auto_repeat:bool,

}

impl KeyboardProperty {
    /// Create new instance of keyboard property with auto repeat to false.
    pub fn new() -> KeyboardProperty {
        KeyboardProperty { auto_repeat: false }
    }
}