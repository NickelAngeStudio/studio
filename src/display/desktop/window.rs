//! Window abstraction and properties

use std::any::Any;

use super::{provider::WindowProvider };
use super::pointer::{ PointerMode};
use super::event::Event;

 /// Minimum [Window] width allowed.
 pub const WINDOW_MIN_WIDTH : u32 = 1;

 /// Minimum [Window] height allowed.
 pub const WINDOW_MIN_HEIGHT : u32 = 1;

 /// Maximum [Window] width allowed.
 pub const WINDOW_MAX_WIDTH : u32 = 65535;

 /// Maximum [Window] height allowed.
 pub const WINDOW_MAX_HEIGHT : u32 = 65535;



/// [Window] fullscreen mode enumeration.
pub enum FullscreenMode {
    /// Window will be set fullscreen in the current screen this window belong to.
    CurrentScreen,

    /// Window will be set fullscreen in the primary screen.
    PrimaryScreen,

    /// Window will be set fullscreen for entire desktop which can be set across multiple physical screen.
    DesktopScreen,
}

/// Abstraction of a [Window](https://en.wikipedia.org/wiki/Windowing_system#Display_server)
/// and/or [Window manager](https://en.wikipedia.org/wiki/Window_manager) used to create and manage window.
pub trait Window {
    /// Get the window provider managing this window.
    fn get_window_provider(&self) -> WindowProvider;

    /// Pop a window event from the queue.
    fn poll_event(&mut self) -> Event;

    /// Get the count of events that need handling.
    fn get_event_count(&self) -> usize;

    /// Get the current pointer position.
    fn get_pointer_position(&self) -> (i32, i32);

    /// Set the pointer position
    fn set_pointer_position(&mut self, position : (i32, i32));

    /// Get the [Window] pointer mode.
    fn get_pointer_mode(&self);

    /// Set the pointer mode for the [Window] [EventMouse](super::event::EventMouse) events.
    fn set_pointer_mode(&mut self, mode : PointerMode) ;

    /// Hide system default cursor.
    fn hide_pointer(&mut self);

    /// Show system default cursor.
    fn show_pointer(&mut self);

    /// Confine cursor to window, preventing it from exiting boundaries.
    fn confine_pointer(&mut self);

    /// Release cursor from window, allowing it to exit boundaries.
    fn release_pointer(&mut self);

    /// Restore the window, undoing any minimized, maximized and/or fullscreen status.
    fn restore(&mut self);

    /// Get window title.
    fn get_title(&self) -> &str;

    /// Set a new title for the window.
    fn set_title(&mut self, title : &str);

    /// Get current window size.
    fn get_size() ->  (u32, u32);

    /// Set a size for window.
    fn set_size(&mut self, size : (u32, u32));

    /// Get current window position.
    fn get_position() ->  (i32, i32);

     /// Set a position of window.
    fn set_position(&mut self, position : (i32, i32));

    /// Return true if window is set to fullscreen.
    fn is_fullscreen() -> bool;

    /// Set the window as fullscreen according to [FullscreenMode].
    fn set_fullscreen(&mut self, fs_mode : FullscreenMode);

    /// Perform sync with the display server / window manager.
    fn sync(&self);

    /// Get self as Any, use for downcast. 
    /// 
    /// Implementation only need to return self.
    fn as_any(&self) -> &dyn Any;

    /// Get self as mut Any, use for downcast. 
    /// 
    /// Implementation only need to return mut self.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
