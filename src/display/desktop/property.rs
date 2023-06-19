use crate::{display::desktop::screen::Screen};

use super::window::Window;

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

/// [Window] fullscreen mode enumeration.
#[derive(Clone)]
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
#[derive(Clone)]
pub enum WindowPositionOption {
    /// Position window on desktop from an absolute pair of x,y coordinates.
    Desktop((i32, i32)),

    /// Position window on a specific screen from an absolute pair of x,y coordinates.
    Screen(Screen, (i32, i32)),

    /// Position window relative to parent window. If no parent, will be at desktop 0,0.
    Parent((i32, i32)),

    /// Position window in the center of given screen.
    CenterScreen(Screen),

    /// Position window in the center of parent window. If no parent, will be at desktop 0,0.
    CenterParent,
}

/// Contains keyboard properties that can be set.
pub enum KeyboardPropertySet {

    /// Key will be repeated when pressed down.
    EnableAutoRepeat,


    /// Key won't be repeated when pressed down.
    DisableAutoRepeat,

}

/// [Window](super::window::Window) pointer properties such as mode, position, etc.
pub enum PointerPropertySet {

    /// [PointerMode] used for [EventMouse](super::event::EventMouse) events.
    Mode(PointerMode),

    /// Current cursor position on the window.
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

/// Enumeration of possible sub window display options.
#[derive(Clone, Copy)]
pub enum SubWindowOption {

    /// Child is showed as normal window.
    Normal,

    /// Child is showed always on top of parent.
    Top,

    /// Child is showed on top of parent and prevent any parent event until closed.
    Modal

}

/// Enumeration of possible [Window](super::window::Window) properties that can be set.
pub enum WindowPropertySet<'window> {

    /// Set the window parent from a reference cell. Sub window are showed according to [SubWindowOption].
    /// When closing a parent, all sub window must also be closed.
    /// 
    /// Note : A window cannot be it's own parent nor can it become the subwindow of his subwindows.
    SetParent(&'window Window<'window>, SubWindowOption),

    /// Remove window from parent, making it a parentless window.
    RemoveParent,

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
pub struct WindowProperty<'window> {

    /// [Window] parent reference must be contained in a reference counter.
    pub(super) parent : Option<(&'window Window<'window>, SubWindowOption)>,

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

    /// Window is locked. Usually by showing a modal window.
    pub locked:bool,
}

impl<'window> WindowProperty<'window>{
    pub(crate) fn new() -> WindowProperty<'window> {
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
            locked: false,
            relative_position: WindowPositionOption::Desktop((0,0)),
            parent: None,
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


/// Contains keyboard properties.
pub struct KeyboardProperty {

    /// If true, key will be repeated when pressed down.
    pub auto_repeat:bool,

}

impl KeyboardProperty {
    /// Create new instance of keyboard property with auto repeat to false.
    pub(crate) fn new() -> KeyboardProperty {
        KeyboardProperty { auto_repeat:false }
    }
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
/// Return Ok(position) on success, DisplayError::PositionNoParent on failure.
pub fn get_absolute_position_from_relative(size : (u32, u32), parent: Option<(&Window<'_>, SubWindowOption)>, option : &WindowPositionOption) -> (i32, i32) {

    // Take value from parent if any or init at (0,0) if None
    let (parent_pos,parent_size) = match parent {
        Some(parent) => (parent.0.get_properties().position, parent.0.get_properties().size),
        None => ((0,0),(0,0)),
    };

    match option {
        // Desktop position is already an absolute.
        WindowPositionOption::Desktop(position) => *position,

        // Add screen offset to position
        WindowPositionOption::Screen(screen, position) =>
            (screen.get_extended_position().0 + position.0, screen.get_extended_position().1 + position.1),

        // Add parent position. Will raise error if no parent.
        WindowPositionOption::Parent(position) => {
            (parent_pos.0 + position.0, parent_pos.1 + position.1)
        },

        // Center to the screen.
        WindowPositionOption::CenterScreen(screen) => {
            let screen_pos = screen.get_extended_position();
            let screen_size = screen.get_current_resolution();

            (screen_pos.0 + ((screen_size.0 - size.0) / 2) as i32, screen_pos.1 + ((screen_size.1 - size.1) / 2) as i32)
        },


        WindowPositionOption::CenterParent => {
            (parent_pos.0 + ((parent_size.0 - size.0) / 2) as i32, parent_pos.1 + ((parent_size.1 - size.1) / 2) as i32)
        },
    }
}