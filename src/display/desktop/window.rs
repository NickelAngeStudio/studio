//! Window abstraction and properties

use std::any::Any;

use crate::error::StudioError;

use super::{provider::WindowProvider };
use super::pointer::{ PointerMode, PointerProperty};
use super::keyboard::KeyboardProperty;
use super::event::Event;

 /// Minimum [Window] width allowed.
 pub const WINDOW_MIN_WIDTH : u32 = 1;

 /// Minimum [Window] height allowed.
 pub const WINDOW_MIN_HEIGHT : u32 = 1;

 /// Maximum [Window] width allowed.
 pub const WINDOW_MAX_WIDTH : u32 = 65535;

 /// Maximum [Window] height allowed.
 pub const WINDOW_MAX_HEIGHT : u32 = 65535;

 /// [Window] properties.
 pub struct WindowProperty {

    /// Window title
    pub title : String,

    /// Position of window as pair of i32(x,y)
    pub position : (i32, i32),

    /// Size of window as pair of u32 (width, height).
    pub size : (u32, u32),

    /// Window center,
    pub center : (i32, i32),

    /// Window is minimized
    pub is_minimized : bool,

    /// Window is maximized
    pub is_maximized : bool,

    /// Window is fullscreen
    pub is_fullscreen : bool,

}

impl WindowProperty {
    /// Create a new instance of [KWindowProperty] with default values from position and size.
    pub fn new(position : (i32, i32), size : (u32, u32)) -> WindowProperty {
        WindowProperty{ 
            title: String::new(), 
            position, 
            size, 
            center: (size.0 as i32 / 2, size.1 as i32 / 2), 
            is_minimized: false, 
            is_maximized: false, 
            is_fullscreen: false 
        }
    }

    /// Returns true if size if within MIN and MAX.
    pub fn is_size_within_boundaries(size : (u32, u32)) -> bool {

        if size.0 >= WINDOW_MIN_WIDTH && size.0 <= WINDOW_MAX_WIDTH && size.1 >= WINDOW_MIN_HEIGHT && size.1 <= WINDOW_MAX_HEIGHT {
            // Withing boundaries
            true
        } else {
            // Boundaries overflow
            false
        }

    }
}



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

    /// Get [Window] properties.
    fn get_window_properties(&self) -> &WindowProperty;

    /// Get [PointerProperty] for window.
    fn get_pointer_properties(&self) -> &PointerProperty;

    /// Get [KeyboardProperty] for window.
    fn get_keyboard_properties(&self) -> &KeyboardProperty;

    /// Set the pointer position
    fn set_pointer_position(&mut self, position : (i32, i32));

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

    /// Enable auto repeat of keyboard keys when pressed down. Disabled by default.
    fn enable_autorepeat(&mut self);

    /// Disable auto repeat of keyboard keys when pressed down.
    fn disable_autorepeat(&mut self);

    /// Restore the window, undoing any minimized, maximized and/or fullscreen status.
    fn restore(&mut self);

    /// Set a new title for the window.
    fn set_title(&mut self, title : &str);

    /// Set a size for window. 
    /// 
    /// Return Ok() with new size on success, StudioError::Display(SizeError) on error.
    fn set_size(&mut self, size : (u32, u32)) -> Result<(u32, u32), StudioError>;

     /// Set a position of window.
    fn set_position(&mut self, position : (i32, i32));

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
