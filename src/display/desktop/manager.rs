use cfg_boost::target_cfg;

use crate::error::StudioError;

use super::{event::Event, provider::WindowProvider, property::{WindowProperty} };


/// Manager that manage the [Window].
/// 
/// Each property set returns either true or false depending if window need to be recreate.
pub trait WindowManager {
    /// Create a new WindowManager instance.
    fn new() -> Result<Self, StudioError> where Self : Sized;

    /// Get the window provider id
    fn get_window_provider(&self) -> WindowProvider;

    /// Pop an event from the manager.
    fn poll_event(&mut self) -> Event ;

    /// Show the window according to [WindowProperty]
    fn show(&mut self, property : &WindowProperty);

    /// Force close the window.
    fn close(&mut self);

    /// Hide the window.
    fn hide(&mut self);

    /// Get the OS Window manager window handle.
    fn get_window_handle(&self) -> Option<*const usize>;

    /// Push an event that will be poll during poll_event.
    fn push_event(&mut self, event: Event);

    target_cfg! {
        linux => {
            /// Get the OS Window manager display handle.
            fn get_display_handle(&self) -> Option<*const usize>;
        }
    }

    /// Set the window title.
    fn set_title(&mut self, title : &String) -> bool;

    /// Set window absolute position.
    fn set_position(&mut self, position : (i32,i32)) -> bool;

    /// Set window size.
    fn set_size(&mut self, size : &(u32,u32)) -> bool;

    /// Show window decoration such as title bar, buttons, etc...
    fn show_decoration(&mut self) -> bool;

    /// Hide window decoration such as title bar, buttons, etc...
    fn hide_decoration(&mut self) -> bool;

    /// Minimize window into taskbar.
    fn minimize(&mut self) -> bool;

    /// Maximize window.
    fn maximize(&mut self) -> bool;

    /// Setting window fullscreen trigger 
    /// window recreate.
    /// 
    /// This need to be overriden if not the case.
    fn set_fullscreen(&mut self) -> bool { true }

    /// Enable keyboard key auto-repeat when hold.
    fn enable_autorepeat(&mut self) -> bool;

    /// Disable keyboard key auto-repeat when hold.
    fn disable_autorepeat(&mut self) -> bool;

    /// Set the pointer position relative to the window.
    fn set_pointer_position(&mut self, position : &(i32, i32)) -> bool;

    /// Show the pointer, making it visible.
    fn show_pointer(&mut self) -> bool;

    /// Hide the pointer, making it invisible.
    fn hide_pointer(&mut self) -> bool;

    /// Confine the pointer to window boundaries, preventing escape.
    fn confine_pointer(&mut self) -> bool;

    /// Release the pointer from window boundaries, allowing escape.
    fn release_pointer(&mut self) -> bool;


}