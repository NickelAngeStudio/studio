use crate::display::desktop::{manager::{WindowManager}, Window};

/// Lazy Static list of keycode identity. Must be initialized by the first window call.
pub(crate) static mut KEYCODE_IDENTITY : Option<KeyCodeIdentityList> = Option::None;

/// Enumeration of possible Keyboard events
#[derive(Debug, Copy, Clone)]
pub enum EventKeyboard {
    // Keyboard key down event. Provides modifier and keycode of key pressed.
    KeyDown(Key),

    // Keyboard key up event. Provides modifier and keycode of key released.
    KeyUp(Key),
}


/// Represent a key on keyboard.
#[derive(Debug, Copy, Clone)]
pub struct Key {
    modifier : u8,
    pub keycode : u8,
}

impl Key {
    /// Key a new key with modifier.
    pub fn new(modifier : u8, keycode: u8) -> Key {
        Key{ modifier, keycode }
    }

    /// Get the key physical identification.
    pub fn identity(&self) -> KeyIdentity {
        match unsafe { &KEYCODE_IDENTITY } {
            Some(list) => list.get_identity(self.keycode),
            None => KeyIdentity::NOLIST,  // Return that list isnt created.
        }
    }

    /// Get the key char value.
    pub fn get_char(&self) -> char {
        todo!()
        //window.manager.get_key_char(self)
    }

    /// Either left or right shift were pressed with key
    pub fn shift(&self) -> bool{
        KeyModifier::SHIFT & self.modifier > 0
    }

    /// Either left or right ctrl were pressed with key
    pub fn ctrl(&self) -> bool{
        KeyModifier::CTRL & self.modifier > 0
    }

    /// Either left or right alt were pressed with key
    pub fn alt(&self) -> bool{
        KeyModifier::ALT & self.modifier > 0
    }

    /// Either left or right meta were pressed with key
    /// 
    /// Reference(s)
    /// https://askubuntu.com/questions/19558/what-are-the-meta-super-and-hyper-keys
    pub fn meta(&self) -> bool{
        KeyModifier::META & self.modifier > 0
    }

    /// Either left or right super(command) were pressed with key
    /// 
    /// Reference(s)
    /// https://askubuntu.com/questions/19558/what-are-the-meta-super-and-hyper-keys
    pub fn command(&self) -> bool{
        KeyModifier::COMMAND & self.modifier > 0
    }

    /// Either left or right hyper were pressed with key
    /// 
    /// Reference(s)
    /// https://askubuntu.com/questions/19558/what-are-the-meta-super-and-hyper-keys
    pub fn hyper(&self) -> bool{
        KeyModifier::HYPER & self.modifier > 0
    }

    /// Indicate if the caps lock was on (true) or off (false) when key was pressed.
    pub fn capslock(&self) -> bool{
        KeyModifier::CAPSLOCK & self.modifier > 0
    }

    /// Indicate if the num lock was on (true) or off (false) when key was pressed.
    pub fn numlock(&self) -> bool{
        KeyModifier::CAPSLOCK & self.modifier > 0
    }

}

/// List that contains key Identity from keycodes.
pub struct KeyCodeIdentityList {
    list : Box<[KeyIdentity;u8::MAX as usize]>,
}

impl KeyCodeIdentityList {
    /// Create a new list of identity.
    pub fn new(list : Box<[KeyIdentity;u8::MAX as usize]>) -> KeyCodeIdentityList {
        KeyCodeIdentityList{list}
    }

    pub fn get_identity(&self, keycode : u8) -> KeyIdentity {
        self.list[keycode as usize]
    }
}

/// Physical key identity according to XKB specifications.
/// 
/// Key not represented here will be returned as NOID.
/// 
/// Reference(s)
/// Diagram of keys : <https://abaines.me.uk/img/xkb.png>
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyIdentity {

    /// Keylist was not initialized
    NOLIST,

    /// Key as no known XKB physical ID.
    NOID,

    /***************
    * TOP ROW KEYS *
    ***************/
    /// Escape key
    ESC,

    /// F1 key
    FK01,

    /// F2 key
    FK02,

    /// F3 key
    FK03,

    /// F4 key
    FK04,

    /// F5 key
    FK05,

    /// F6 key
    FK06,

    /// F7 key
    FK07,

    /// F8 key
    FK08,

    /// F9 key
    FK09,

    /// F10 key
    FK10,

    /// F11 key
    FK11,

    /// F12 key
    FK12,

    /// Print key
    PRSC,

    /// Screen lock key
    SCLK,

    /// Pause key
    PAUS,


     /**************
    * 2nd ROW KEYS *
    ***************/
    /// Tilde key
    TLDE,

    /// Numeric 1st key
    AE01,

    /// Numeric 2nd key
    AE02,

    /// Numeric 3rd key
    AE03,

    /// Numeric 4th key
    AE04,

    /// Numeric 5th key
    AE05,

    /// Numeric 6th key
    AE06,

    /// Numeric 7th key
    AE07,

    /// Numeric 8th key
    AE08,

    /// Numeric 9th key
    AE09,

    /// Numeric 10th key
    AE10,

    /// Numeric 11th key
    AE11,

    /// Numeric 12th key
    AE12,

    /// Backspace physical key
    BKSP,


     /**************
    * 3rd ROW KEYS *
    ***************/
    /// Tab key
    TAB,

    /// Top char 1st key
    AD01,

    /// Top char 2nd key
    AD02,

    /// Top char 3rd key
    AD03,

    /// Top char 4th key
    AD04,

    /// Top char 5th key
    AD05,

    /// Top char 6th key
    AD06,

    /// Top char 7th key
    AD07,

    /// Top char 8th key
    AD08,

    /// Top char 9th key
    AD09,

    /// Top char 10th key
    AD10,

    /// Top char 11th key
    AD11,

    /// Top char 12th key
    AD12,

    /// Backslash physical key
    BKSL,

     /**************
    * 4th ROW KEYS *
    ***************/
    /// Capslock key
    CAPS,

    /// Middle char 1st key
    AC01,

    /// Middle char 2nd key
    AC02,

    /// Middle char 3rd key
    AC03,

    /// Middle char 4th key
    AC04,

    /// Middle char 5th key
    AC05,

    /// Middle char 6th key
    AC06,

    /// Middle char 7th key
    AC07,

    /// Middle char 8th key
    AC08,

    /// Middle char 9th key
    AC09,

    /// Middle char 10th key
    AC10,

    /// Middle char 11th key
    AC11,

    /// Return/Enter key
    RTRN,

     /**************
    * 5th ROW KEYS *
    ***************/
    /// Left shift key
    LFSH,

    /// Bottom char 1st key
    AB01,

    /// Bottom char 2nd key
    AB02,

    /// Bottom char 3rd key
    AB03,

    /// Bottom char 4th key
    AB04,

    /// Bottom char 5th key
    AB05,

    /// Bottom char 6th key
    AB06,

    /// Bottom char 7th key
    AB07,

    /// Bottom char 8th key
    AB08,

    /// Bottom char 9th key
    AB09,

    /// Bottom char 10th key
    AB10,

    /// Right shift key
    RTSH,

    /***************
    * BTM ROW KEYS *
    ***************/
    /// Left ctrl key
    LCTL,

    /// Left alt key
    LALT,

    /// Space key
    SPCE,

    /// Right alt key
    RALT,

    /// Right ctrl key
    RCTL,

    /******************
    * NAVIGATION KEYS *
    ******************/
    /// Insert key
    INS,

    /// Home key
    HOME,

    /// Page up key
    PGUP,

    /// Delete key
    DELE,

    /// End key
    END,

    /// Page down key
    PGDN,

    /// Up arrow key
    UP,

    /// Left arrow key
    LEFT,

    /// Down arrow key
    DOWN,

    /// Right arrow key
    RGHT,

     /*************
    * KEYPAD KEYS *
    **************/
    /// Numlock key
    NMLK,

    /// Keypad division key
    KPDV,

    /// Keypad multiply key
    KPMU,

    /// Keypad substract key
    KPSU,

    /// Keypad add key
    KPAD,

    /// Keypad enter key
    KPEN,

    /// Keypad divider/delete key
    KPDL,

    /// Keypad 0 key
    KP0,

    /// Keypad 1 key
    KP1,

    /// Keypad 2 key
    KP2,

    /// Keypad 3 key
    KP3,

     /// Keypad 4 key
    KP4,

     /// Keypad 5 key
    KP5,

     /// Keypad 6 key
    KP6,

     /// Keypad 7 key
    KP7,

     /// Keypad 8 key
    KP8,

     /// Keypad 9 key
    KP9,
}

/// Keyboard modifiers bit shifter.
#[allow(non_snake_case)]
pub mod KeyModifier {
    pub const SHIFT:u8 = 1 << 0;
    pub const CTRL:u8 = 1 << 1;
    pub const ALT:u8 = 1 << 2;
    pub const META:u8 = 1 << 3;
    pub const COMMAND:u8 = 1 << 4;
    pub const HYPER:u8 = 1 << 5;
    pub const CAPSLOCK:u8 = 1 << 6;
    pub const NUMLOCK:u8 = 1 << 7;
}
