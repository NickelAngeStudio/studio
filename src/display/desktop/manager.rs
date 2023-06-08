use crate::error::StudioError;

use super::{event::Event, screen::ScreenList};

pub(crate) struct WindowManagerShowParameters {

}

pub trait WindowManager {
    /// Create a new instance of Window manager.
    /// 
    /// Return Ok(Self) on success and Err([StudioError]) on error.
    fn new() -> Result<Self, StudioError> where Self: Sized;

    /// Pop an event from the manager.
    fn poll_event(&mut self) -> Event ;

    /// Show the window according to parameters
    fn show(&mut self, parameters : WindowManagerShowParameters);

    /// Restore window to previous state.
    fn restore(&mut self);

    /// Force close the window.
    fn close(&mut self);

    /// Hide the window.
    fn hide(&mut self);

    /// Get the list of hardware screen display.
    fn get_screen_list(&self) -> &ScreenList;

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

    

}