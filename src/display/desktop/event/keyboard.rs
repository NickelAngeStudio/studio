use crate::display::desktop::manager::{WindowManagerType, WindowManager};

/// Enumeration of possible Keyboard events
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EventKeyboard {
    // Keyboard key down event of direct input mode. Provides keycode as u32.
    KeyDown(u32),

    // Keyboard key down event of direct input mode. Provides keycode as u32.
    KeyUp(u32),

    // KeyPress happens provides [Key] struct
    KeyPress(Key),
}


/// Represent a key on keyboard.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Key {

    /// Keyboard state when captured
    pub(crate) state : u32,  

    /// Keyboard keycode
    pub keycode : u32,
    
    /// UTF8 Character typed on the key press if available.
    pub character : Option<char>,
}

impl Key {
    /// Create a new KeyPress Key entry from state, keycode and character.
    pub fn new(state : u32, keycode : u32, character : Option<char>) -> Key {
        Key { state, keycode, character }
    }
    
    /// Either left or right shift were pressed with key
    pub fn is_shift_down(&self) -> bool{
        WindowManagerType::is_key_shift_down(self.state)
    }

    /// Either left or right ctrl were pressed with key
    pub fn is_ctrl_down(&self) -> bool{
        WindowManagerType::is_key_ctrl_down(self.state)
    }

    /// Either left or right alt were pressed with key
    pub fn is_alt_down(&self) -> bool{
        WindowManagerType::is_key_alt_down(self.state)
    }

    /// Either left or right meta were pressed with key
    /// 
    /// Reference(s)
    /// https://askubuntu.com/questions/19558/what-are-the-meta-super-and-hyper-keys
    pub fn is_meta_down(&self) -> bool{
        WindowManagerType::is_key_meta_down(self.state)
    }

    /// Either left or right super(command) were pressed with key
    /// 
    /// Reference(s)
    /// https://askubuntu.com/questions/19558/what-are-the-meta-super-and-hyper-keys
    pub fn is_command_down(&self) -> bool{
        WindowManagerType::is_key_command_down(self.state)
    }

    /// Either left or right hyper were pressed with key
    /// 
    /// Reference(s)
    /// https://askubuntu.com/questions/19558/what-are-the-meta-super-and-hyper-keys
    pub fn is_hyper_down(&self) -> bool{
        WindowManagerType::is_key_hyper_down(self.state)
    }

    /// Indicate if the caps lock was on (true) or off (false) when key was pressed.
    pub fn is_capslock_on(&self) -> bool{
        WindowManagerType::is_capslock_on(self.state)
    }

    /// Indicate if the num lock was on (true) or off (false) when key was pressed.
    pub fn is_numlock_on(&self) -> bool{
        WindowManagerType::is_numlock_on(self.state)
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

    // All code according to XKB evdev keycodes (/usr/share/X11/xkb/keycodes/evdev)
    LSGT,TLDE,AE01,AE02,AE03,AE04,AE05,AE06,AE07,AE08,AE09,AE10,AE11,AE12,BKSP,TAB,AD01,AD02,AD03,AD04,AD05,AD06,AD07,AD08,AD09,AD10,AD11,AD12,BKSL,RTRN,CAPS,AC01,AC02,
    AC03,AC04,AC05,AC06,AC07,AC08,AC09,AC10,AC11,LFSH,AB01,AB02,AB03,AB04,AB05,AB06,AB07,AB08,AB09,AB10,RTSH,LALT,LCTL,SPCE,RCTL,RALT,LWIN,RWIN,COMP,ESC,FK01,FK02,FK03,
    FK04,FK05,FK06,FK07,FK08,FK09,FK10,FK11,FK12,PRSC,SCLK,PAUS,INS,HOME,PGUP,DELE,END,PGDN,UP,LEFT,DOWN,RGHT,NMLK,KPDV,KPMU,KPSU,KP7,KP8,KP9,KPAD,KP4,KP5,KP6,KP1,KP2,
    KP3,KPEN,KP0,KPDL,KPEQ,FK13,FK14,FK15,FK16,FK17,FK18,FK19,FK20,FK21,FK22,FK23,FK24,HKTG,AB11,HENK,MUHE,AE13,KATA,HIRA,JPCM,HNGL,HJCV,MUTE,VOLD,VOLU,POWR,STOP,AGAI,
    PROP,UNDO,FRNT,COPY,OPEN,PAST,FIND,CUT,HELP,LNFD,I120,I126,I128,I129,I147,I148,I149,I150,I151,I152,I153,I154,I155,I156,I157,I158,I159,I160,I161,I162,I163,I164,I165,
    I166,I167,I168,I169,I170,I171,I172,I173,I174,I175,I176,I177,I178,I179,I180,I181,I182,I183,I184,I185,I186,I187,I188,I189,I190,I208,I209,I210,I211,I212,I213,I214,I215,
    I216,I217,I218,I219,I220,I221,I222,I223,I224,I225,I226,I227,I228,I229,I230,I231,I232,I233,I234,I235,I236,I237,I238,I239,I240,I241,I242,I243,I244,I245,I246,I247,I248,
    I249,I250,I251,I252,I253,I254,I255,LVL3,MDSW,ALT,META,SUPR,HYPR,
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
