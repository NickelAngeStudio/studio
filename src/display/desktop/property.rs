use crate::error::StudioError;

use super::{screen::Screen, manager::WindowManager};


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

/// Enumeration of possible window positions when setting position.
pub enum WindowPosition {
    /// Position window on desktop from an absolute pair of x,y coordinates.
    Desktop((i32, i32)),

    /// Position window on primary screen from an absolute pair of x,y coordinates.
    PrimaryScreen((i32, i32)),

    /// Position window on a specific screen from an absolute pair of x,y coordinates.
    Screen(Screen, (i32, i32)),

    /// Position window in the center of the primary screen.
    CenterPrimaryScreen,

    /// Position window in the center of given screen.
    CenterScreen(Screen),

    /// Position window in the center of parent window. If no parent, will be center of default screen.
    CenterParent,
}

/// [Window] properties.
pub struct WindowProperty<'window, WM : WindowManager> {

    /// Window manager those properties applies to
    manager : &'window WM,

    /// Window pointer properties
    pub(crate) pointer : PointerProperty<'window, WM>,

    /// Window pointer properties
    pub(crate) keyboard : KeyboardProperty<'window, WM>,

    /// Window title
    pub(crate) title : String,

    /// Position of window as pair of i32(x,y)
    pub(crate) position : (i32, i32),

    /// Size of window as pair of u32 (width, height).
    pub(crate) size : (u32, u32),

    /// Window center,
    pub(crate) center : (i32, i32),

    /// Window is minimized
    pub(crate) minimized : bool,

    /// Window is maximized
    pub(crate) maximized : bool,

    /// Window is fullscreen
    pub(crate) fullscreen : bool,

    // Window is visible.
    pub(crate) visible: bool,
}

impl<'window, WM> WindowProperty<'window, WM> where WM: WindowManager{
    /// Create a new instance of [KWindowProperty] with default values from position and size.
    pub(crate) fn new(manager: &WM) -> WindowProperty<'window, WM> {
        WindowProperty{ 
            title: String::new(), 
            position : (0,0), 
            size: (DEFAULT_WIDTH, DEFAULT_HEIGHT), 
            center: (DEFAULT_WIDTH as i32 / 2, DEFAULT_HEIGHT as i32 / 2), 
            minimized: false, 
            maximized: false, 
            fullscreen: false,
            manager,
            pointer: PointerProperty::new(manager),
            keyboard: KeyboardProperty::new(manager),
            visible: false, 
        }
    }

    /// Returns true if size if within MIN and MAX.
    fn is_size_within_boundaries(&self, size : (u32, u32)) -> bool {

        if size.0 >= WINDOW_MIN_WIDTH && size.0 <= WINDOW_MAX_WIDTH && size.1 >= WINDOW_MIN_HEIGHT && size.1 <= WINDOW_MAX_HEIGHT {
            // Withing boundaries
            true
        } else {
            // Boundaries overflow
            false
        }

    }

    get_pointer : PointerProperty<'window, WM>,

    get_pointer_mut : PointerProperty<'window, WM>,

    /// Window pointer properties
    get_keyboard : KeyboardProperty<'window, WM>,

    get_keyboard_mut : KeyboardProperty<'window, WM>,

    /// Get [Window](super::window::Window) title.
    pub fn get_title(&self) -> String {
        self.title
    }

    /// Set a new title for the window.
    pub fn set_title(&mut self, title : &str){

        self.title = String::from(title);

        if self.visible {
            self.manager.set_title(title);
        }
    }

    /// Get [Window](super::window::Window) size.
    pub fn get_size(&self) -> (u32, u32) {
        self.size
    }

    /// Get [Window](super::window::Window) center.
    pub fn get_center(&self) -> (i32, i32) {
        self.center
    }

    /// Set a size for window. 
    /// 
    /// Return Ok() with new size on success, StudioError::Display(SizeError) on error.
    pub fn set_size(&mut self, size : (u32, u32)) -> Result<(u32, u32), StudioError>{

        if self.is_size_within_boundaries(size) {
            self.size = size;

            if self.visible {
                self.manager.set_size(size);
            }

            Ok(size)
        } else {
            Err(StudioError::Display(crate::display::error::DisplayError::SizeError))
        }

    }

    /// Get [Window](super::window::Window) position.
    pub fn get_position(&self) -> (i32, i32) {
        self.position
    }

    /// Set a position of window.
    pub fn set_position(&mut self, position : WindowPosition){

        // Deduce position from enumeration.
        match position {
            // Absolute positionning from a pair of coordinates.
            WindowPosition::Desktop(position) => self.position = position,
            
            // Absolute position on default screen
            WindowPosition::PrimaryScreen(position) => todo!(),

            // Absolute position on given screen
            WindowPosition::Screen(screen, position) => todo!(),

            // Position in the center of default screen.
            WindowPosition::CenterPrimaryScreen => todo!(),
            
            // Position in the center of the screen.
            WindowPosition::CenterScreen(screen) => todo!(),
            
            // Position in the center of the parent.
            WindowPosition::CenterParent => todo!(),
        }

        if self.visible {
            self.manager.set_position(self.position);
        }
    }

    /// Window is minimized
    pub fn is_minimized(&self) -> bool{
        self.minimized
    }

    /// Window is maximized
    pub fn is_maximized(&self) -> bool{
        self.maximized
    }

    /// Window is fullscreen
    pub fn is_fullscreen(&self) -> bool{
        self.fullscreen
    }

    // Window is visible.
    pub fn is_visible(&self) -> bool{
        self.visible
    }

}


/// Contains keyboard properties.
pub struct KeyboardProperty<'window, WM : WindowManager> {
    /// Window manager those properties applies to
    manager : &'window WM,

    /// If true, key will be repeated when pressed down.
    pub(crate) auto_repeat:bool,

}

impl<'window, WM> KeyboardProperty<'window, WM> where WM: WindowManager {
    /// Create new instance of keyboard property with auto repeat to false.
    pub(crate) fn new(manager: &WM) -> KeyboardProperty<'window, WM> {
        KeyboardProperty { auto_repeat: false, manager }
    }

    /// Enable auto repeat of keyboard keys when pressed down. Disabled by default.
    pub fn enable_autorepeat(&mut self){
        self.auto_repeat = true;
    }

    /// Disable auto repeat of keyboard keys when pressed down.
    pub fn disable_autorepeat(&mut self){
        self.auto_repeat = false;
    }

    /// Returns if [Window](super::window::Window) auto key repeat is on.
    pub fn is_autorepeat(&self) -> bool {
        self.auto_repeat
    }
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

/// [Window](super::window::Window) cursor properties such as mode, position, etc.
pub struct PointerProperty<'window, WM : WindowManager> {
    /// Window manager those properties applies to
    manager : &'window WM,

    /// [PointerMode] used for [EventMouse](super::event::EventMouse) events.
    pub(crate) mode : PointerMode,

    /// Current cursor position on the window.
    pub(crate) position : (i32, i32),

    /// Indicate if cursor is visible or hidden.
    pub(crate) visible : bool,

    /// Indicate if cursor is confined to the window boundaries or not.
    pub(crate) confined : bool, 
}


impl<'window, WM> PointerProperty<'window, WM> where WM: WindowManager {
    /// Create a new [PointerProperty] with default values.
    pub(crate) fn new(manager: &WM) -> PointerProperty<'window, WM> {
        PointerProperty{ 
            manager,
            mode: PointerMode::Cursor, 
            position: (0,0), 
            visible: true, 
            confined: false,
        }
    }

    /// Get the pointer position.
    pub fn get_position(&self) -> (i32, i32) {
        self.position
    }

    /// Set the pointer position
    pub fn set_position(&mut self, position : (i32, i32)){
        self.position = position;
        self.manager.set_pointer_position(position);
    }

    /// Get the pointer mode
    pub fn get_mode(&self) -> PointerMode {
        return self.mode
    }

    /// Set the pointer mode for the [Window] [EventMouse](super::event::EventMouse) events.
    pub fn set_mode(&mut self, mode : PointerMode){
        self.mode = mode;
    }

    /// Hide system default cursor.
    pub fn hide(&mut self){
        self.visible = false;
        self.manager.hide_pointer();
    }

    /// Show system default cursor.
    pub fn show(&mut self){
        self.visible = true;
        self.manager.show_pointer();
    }

    /// Return true if pointer is currently visible for that [Window](super::window::Window).
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Confine pointer to window, preventing it from exiting boundaries.
    pub fn confine(&mut self){
        self.confined = true;
        self.manager.confine_pointer();
    }

    /// Release pointer from window, allowing it to exit boundaries.
    pub fn release(&mut self){
        self.confined = false;
        self.manager.release_pointer();
    }

    /// Return True if pointer is confined in current window.
    pub fn is_confined(&self) -> bool {
        self.confined
    }

    /// Index of the left button
    pub fn get_left_button_index(&self) -> u32 {
        self.manager.get_left_button_index()
    }

    /// Index of the right button
    pub fn get_right_button_index(&self) -> u32 {
        self.manager.get_right_button_index()
    }

    /// Index of the middle button
    pub fn get_middle_button_index(&self) -> u32 {
        self.manager.get_middle_button_index()
    }

    /// Index of the next button
    pub fn get_next_button_index(&self) -> u32 {
        self.manager.get_next_button_index()
    }

    /// Index of the previous button
    pub fn get_previous_button_index(&self) -> u32 {
        self.manager.get_previous_button_index()
    }

    /// Index of the scroll up button
    pub fn get_scroll_up_index(&self) -> u32 {
        self.manager.get_scroll_up_index()
    }

    /// Index of the scroll down button
    pub fn get_scroll_down_index(&self) -> u32 {
        self.manager.get_scroll_down_index()
    }

    /// Index of the scroll left button
    pub fn get_scroll_left_index(&self) -> u32 {
        self.manager.get_scroll_left_index()
    }

    /// Index of the scroll right button
    pub fn get_scroll_right_index(&self) -> u32 {
        self.manager.get_scroll_right_index()
    }



}