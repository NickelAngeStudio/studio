use cfg_boost::{target_cfg, match_cfg};

use crate::{display::desktop::screen::Screen};

use super::window::{Window, WindowShowOption};

 /// Minimum [Window] width allowed.
pub const WINDOW_MIN_WIDTH : u32 = 1;

/// Minimum [Window] height allowed.
pub const WINDOW_MIN_HEIGHT : u32 = 1;

/// Maximum [Window] width allowed.
pub const WINDOW_MAX_WIDTH : u32 = 65535;

/// Maximum [Window] height allowed.
pub const WINDOW_MAX_HEIGHT : u32 = 65535;

/// Default [Window] width.
pub const DEFAULT_WIDTH : u32 = 640;

/// Default [Window] height.
pub const DEFAULT_HEIGHT : u32 = 480;

/*
/// [Window] event wait mode.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WindowEventWaitMode {
    /// This mode is more suitable for games and interfaces that must refresh
    /// often. This mode is MANDATORY if you want to achieve [IMMEDIATE](https://en.wikipedia.org/wiki/Immediate_mode_GUI) user interfaces.
    NeverWait,

    /// This mode will lock the window thread until an event occurred. Takes way less
    /// computing power and is suited for applications. This mode is MANDATORY if you want to 
    /// achieve [RETAINED](https://en.wikipedia.org/wiki/Retained_mode) user interfaces.
    AlwaysWait,
}
*/

/// [Window] fullscreen mode enumeration.
#[derive(Debug, PartialEq, Clone)]
pub enum FullScreenMode {

    /// Window will be set fullscreen in the current screen this window belong to.
    Current,

    /// Window will be set fullscreen in the primary screen.
    Primary,

    /// Window will be set fullscreen for entire desktop which can be set across multiple physical screen.
    Desktop,

    /// Window will be set fullscreen for the specified screen
    Screen(Screen)
}

/// Enumeration of possible window positions when setting position.
#[derive(Debug, PartialEq, Clone)]
pub enum WindowPositionOption {
    /// Position window on desktop from an absolute pair of x,y coordinates.
    Desktop((i32, i32)),

    /// Position window on a specific screen from an absolute pair of x,y coordinates.
    Screen(Screen, (i32, i32)),

    /// Position window in the center of given screen.
    CenterScreen(Screen),

    /// Position window relative to parent window. If no parent, will be at desktop 0,0.
    #[cfg(not(feature = "immediate"))]
    Parent((i32, i32)),

    /// Position window in the center of parent window. If no parent, will be at desktop 0,0.
    #[cfg(not(feature = "immediate"))]
    CenterParent,
}

/// Contains keyboard properties that can be set.
pub enum KeyboardPropertySet {

    /// Set [KeyboardMode].
    SetMode(KeyboardMode),

    /// Enable key auto-repeat
    EnableAutoRepeat,


    /// Enable key auto-repeat
    DisableAutoRepeat,



}

/// [Window](super::window::Window) pointer properties such as mode, position, etc.
pub enum PointerPropertySet {

    /// [PointerMode] used for [EventMouse](super::event::EventMouse) events.
    Mode(PointerMode),

    /// Set cursor position on the window.
    Position((i32, i32)),

    /// Show cursor
    Show,

    /// Hide cursor
    Hide,

    /// Confine cursor in window
    Confine,

    /// Release cursor from window
    Release,
}

/// Enumeration of possible [Window](super::window::Window) pointer mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PointerMode {
    /// [EventMouse](super::event::EventMouse) events will give the (x,y) location of the cursor on the window. 
    /// 
    /// Usually used for user interfaces interactions.
    Cursor,

    /// [EventMouse](super::event::EventMouse) events will give the (x,y) acceleration of the cursor instead of the position.
    /// 
    /// Usually used for 3d camera and direct mouse inputs.
    Acceleration,
}

/// Enumeration of possible [Window](super::window::Window) properties that can be set.
pub enum WindowPropertySet {

    /// Set window title
    Title(String),

    /// Set window position.
    Position(WindowPositionOption),

    /// Set window size.
    Size((u32, u32)),

    /// Show window decoration
    ShowDecoration,

    /// Hide window decoration
    HideDecoration,

    /// Minimize window
    Minimize,

    /// Maximized window
    Maximized,

    /// Set window fullscreen mode
    Fullscreen(FullScreenMode),

    /// Restore the window, removing minimize, maximize and fullscreen.
    Restore,

    /// Set keyboard property
    Keyboard(KeyboardPropertySet),

    /// Set window pointer properties.
    Pointer(PointerPropertySet),
}   


/// [Window] properties.
pub struct WindowProperty {

    /// [Window] parent reference must be contained in a reference counter.
    pub show_option : Option<WindowShowOption>,

    /// Window pointer properties
    pub pointer : PointerProperty,

    /// Window pointer properties
    pub keyboard : KeyboardProperty,

    /// Window title
    pub title : String,

    /// Absolute position of window as pair of i32(x,y)
    pub position : (i32, i32),

    /// Relative position option used. Doesn't update when moving window.
    pub relative_position : WindowPositionOption,

    /// Size of window as pair of u32 (width, height).
    pub size : (u32, u32),

    /// Window center,
    pub center : (i32, i32),

    /// Show window decoration like title bar, etc...
    pub decoration:bool,

    /// Window is minimized
    pub minimized : bool,

    /// Window is maximized
    pub maximized : bool,

    /// Window is fullscreen
    pub fullscreen : Option<FullScreenMode>,

    /// Window is visible.
    pub visible: bool,

    /// Window is created.
    pub created: bool,
}

impl WindowProperty{
    pub(crate) fn new() -> WindowProperty {
        WindowProperty{ 
            title: String::new(), 
            position : (0,0), 
            size: (DEFAULT_WIDTH, DEFAULT_HEIGHT), 
            center: (DEFAULT_WIDTH as i32 / 2, DEFAULT_HEIGHT as i32 / 2), 
            minimized: false, 
            maximized: false, 
            fullscreen: Option::None,
            pointer: PointerProperty::new(),
            keyboard: KeyboardProperty::new(),
            visible: false,
            created: false,
            decoration: true,
            relative_position: WindowPositionOption::Desktop((0,0)),
            show_option: None,  
        }
    }
    

    /// Returns true if size if within MIN and MAX.
    pub fn is_size_within_boundaries(size : &(u32, u32)) -> bool {

        if size.0 >= WINDOW_MIN_WIDTH && size.0 <= WINDOW_MAX_WIDTH && size.1 >= WINDOW_MIN_HEIGHT && size.1 <= WINDOW_MAX_HEIGHT {
            // Withing boundaries
            true
        } else {
            // Boundaries overflow
            false
        }

    }

}

/// Enumeration of possible keyboard mode for input.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyboardMode {
    /// Direct mode is faster and more suitable for games. Provides [EventKeyboard::KeyUp](super::event::keyboard::EventKeyboard)
    /// and [EventKeyboard::KeyDown](super::event::keyboard::EventKeyboard).
    DirectInput,

    /// Text mode is slower since it provides more information for text entry. Provides [EventKeyboard::KeyPress](super::event::keyboard::EventKeyboard).
    TextInput,
}

/// Contains keyboard properties.
#[derive(Debug, Clone, Copy)]
pub struct KeyboardProperty {

    /// [KeyboardMode] of the keyboard. Use [KeyboardMode::DirectInput] by default.
    pub mode:KeyboardMode,

    /// If enabled, keys are repeated when pressed down. Disabled by default.
    pub auto_repeat : bool,

}

impl KeyboardProperty {
    /// Create new instance of keyboard property with auto repeat to false.
    pub(crate) fn new() -> KeyboardProperty {
        KeyboardProperty { mode : KeyboardMode::DirectInput, auto_repeat: false }
    }
}

/// [Window](super::window::Window) cursor properties such as mode, position, etc.
#[derive(Debug, Clone, Copy)]
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
    pub(crate) fn new() -> PointerProperty {
        PointerProperty{ 
            mode: PointerMode::Cursor, 
            position: (0,0), 
            visible: true, 
            confined: false,
        }
    }
}


/// Get an absolute (x,y) position from a relative position option of a given window.
/// 
/// Parent position and size is always given and should be 0 if no parent is present.
/// 
/// Returns the absolute position of the window.
pub fn get_absolute_position_from_relative(size : (u32, u32), option : &WindowPositionOption, parent_pos_size : ((i32,i32),(u32,u32)) ) -> (i32, i32) {

    match_cfg! {
        !immediate:ft => {  // Retained mode
            match option {
                // Desktop position is already an absolute.
                WindowPositionOption::Desktop(position) => *position,
        
                // Add screen offset to position
                WindowPositionOption::Screen(screen, position) =>
                    (screen.get_extended_position().0 + position.0, screen.get_extended_position().1 + position.1),
        
                // Add parent position. Will raise error if no parent.
                WindowPositionOption::Parent(position) => {
                    (parent_pos_size.0.0 + position.0, parent_pos_size.0.1 + position.1)
                },
        
                // Center to the screen.
                WindowPositionOption::CenterScreen(screen) => {
                    let screen_pos = screen.get_extended_position();
                    let screen_size = screen.get_current_resolution();
        
                    (screen_pos.0 + ((screen_size.0 - size.0) / 2) as i32, screen_pos.1 + ((screen_size.1 - size.1) / 2) as i32)
                },
        
        
                WindowPositionOption::CenterParent => {
                    if parent_pos_size.1.0 > size.0 || parent_pos_size.1.1 > size.1 {   // Only if parent can contain the child
                        (parent_pos_size.0.0 + ((parent_pos_size.1.0 - size.0) / 2) as i32, parent_pos_size.0.1 + ((parent_pos_size.1.1 - size.1) / 2) as i32)
                    } else {
                        (0,0)
                    }
                },
            }
        },
        _ => {  // Immediate mode
            match option {
                // Desktop position is already an absolute.
                WindowPositionOption::Desktop(position) => *position,
        
                // Add screen offset to position
                WindowPositionOption::Screen(screen, position) =>
                    (screen.get_extended_position().0 + position.0, screen.get_extended_position().1 + position.1),
        
                // Center to the screen.
                WindowPositionOption::CenterScreen(screen) => {
                    let screen_pos = screen.get_extended_position();
                    let screen_size = screen.get_current_resolution();
        
                    (screen_pos.0 + ((screen_size.0 - size.0) / 2) as i32, screen_pos.1 + ((screen_size.1 - size.1) / 2) as i32)
                },
            }
        }
        
    }

    
}