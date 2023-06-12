use std::any::Any;

use crate::error::StudioError;

use super::{event::Event, screen::ScreenList, provider::WindowProvider, property::WindowProperty, window::WindowChildDisplayOption};

/// Window manager parameters used to create window.
pub(crate) struct WindowManagerParameter {

    /// Position of window as pair of i32(x,y)
    pub(crate) position : (i32, i32),

    /// Size of window as pair of u32 (width, height).
    pub(crate) size : (u32, u32),

    /// Window is full screen
    pub(crate) full_screen:bool,

    /// Pointer is visible
    pub(crate) pointer_visible:bool,

    /// Pointer is confined
    pub(crate) pointer_confined:bool,

    /// Auto key repeat is enabled
    pub(crate) auto_repeat:bool,

    /// Show window decoration (title bar, etc...)
    pub(crate) decoration:bool,

    /// Window is minimized
    pub(crate) minimized:bool,

    /// Window is maximized
    pub(crate) maximized:bool,
}

impl WindowManagerParameter {

    /// Create a new [WindowManagerParameter] from referenced [windowProperty].
    pub fn from_property(property: &WindowProperty) -> WindowManagerParameter{
        WindowManagerParameter{ 
            position: property.position, 
            size: property.size, 
            full_screen: property.fullscreen, 
            pointer_visible: property.pointer.visible, 
            pointer_confined: property.pointer.confined, 
            auto_repeat: property.keyboard.auto_repeat, 
            decoration: property.decoration,
            minimized: property.minimized,
            maximized: property.maximized, }
    }

    /// Create parameters with default value
    pub fn default() -> WindowManagerParameter {
        WindowManagerParameter{ position: (0,0), size: (0,0), full_screen: false, pointer_visible: true, pointer_confined: false, 
            auto_repeat: false, decoration: true, minimized: false, maximized: false }
    }

}


pub trait WindowManager<'window> {
    /// Create a new instance of Window manager.
    /// 
    /// Return Ok(Self) on success and Err([StudioError]) on error.
    fn new() -> Result<Self, StudioError> where Self: Sized;

    /// Get the window provider id
    fn get_window_provider(&self) -> WindowProvider;

    /// Pop an event from the manager.
    fn poll_event(&mut self) -> Event ;

    /// Send an event to the manager to be retained until poll.
    /// 
    /// NOTE: This won't trigger changes. Sending a resize event won't resize the window.
    fn send_event(&mut self, event : Event);

    /// Show the window according to parameters
    fn show(&mut self, parameters : WindowManagerParameter);

    /// Show as child window according to child format and parent manager.
    fn show_child(&mut self, parent : &dyn WindowManager, parameters : WindowManagerParameter, option : WindowChildDisplayOption);

    /// Restore window to previous state.
    fn restore(&mut self);

    /// Force close the window.
    fn close(&mut self);

    /// Hide the window.
    fn hide(&mut self);

    /// Get the list of hardware screen display.
    fn get_screen_list(&self) -> Result<&ScreenList, StudioError>;

    /// Show window default decoration like title bar, buttons, etc..
    fn show_decoration(&mut self);

    /// Hide window default decoration like title bar, buttons, etc..
    fn hide_decoration(&mut self);

    /// Tell manager to set the new window title
    fn set_title(&mut self, title:&str);

    /// Tell manager to set the window size.
    fn set_size(&mut self, size : (u32, u32));

    /// Tell manager to set the window position.
    fn set_position(&mut self, position : (i32,i32));

    /// Tell manager to set the pointer position.
    fn set_pointer_position(&mut self, position : (i32,i32));

    /// Tell manager to hide the pointer.
    fn hide_pointer(&mut self);

    /// Tell manager to show the pointer.
    fn show_pointer(&mut self);

    /// Tell manager to confine the pointer to the window.
    fn confine_pointer(&mut self);

    /// Tell manager to release the pointer from window.
    fn release_pointer(&mut self);

    /// Tell manager to enable auto key repeat
    fn enable_autorepeat(&mut self);

    /// Tell manager to disable auto key repeat
    fn disable_autorepeat(&mut self);

    /// Get the left button index of the mouse.
    fn get_left_button_index(&self) -> u32;

    /// Get the right button index of the mouse.
    fn get_right_button_index(&self) -> u32;

    /// Get the middle button index of the mouse.
    fn get_middle_button_index(&self) -> u32;

    /// Get the next button index of the mouse.
    fn get_next_button_index(&self) -> u32;

    /// Get the previous button index of the mouse.
    fn get_previous_button_index(&self) -> u32;

    /// Get the scroll up index of the mouse.
    fn get_scroll_up_index(&self) -> u32;

    /// Get the scroll down index of the mouse.
    fn get_scroll_down_index(&self) -> u32;
    
    /// Get the scroll left index of the mouse.
    fn get_scroll_left_index(&self) -> u32;

    /// Get the scroll right index of the mouse.
    fn get_scroll_right_index(&self) -> u32;

    /// Cast self as Any for downcast when needed.
    fn as_any(&'window self) -> &dyn Any;

}