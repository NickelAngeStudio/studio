use std::any::Any;

use crate::display::{provider::WindowProvider, event::WindowEvent};
use super::cursor::{CursorProperty, CursorMode};


 /// Minimum [Display] width allowed.
 pub const WINDOW_MIN_WIDTH : u32 = 1;

 /// Minimum [Display] height allowed.
 pub const WINDOW_MIN_HEIGHT : u32 = 1;

 /// Maximum [Display] width allowed.
 pub const WINDOW_MAX_WIDTH : u32 = 65535;

 /// Maximum [Display] height allowed.
 pub const WINDOW_MAX_HEIGHT : u32 = 65535;


/// [Display] fullscreen mode enumeration.
pub enum FullscreenMode {
    /// Window will be set fullscreen in the current screen this window belong to.
    CurrentScreen,

    /// Window will be set fullscreen in the primary screen.
    PrimaryScreen,

    /// Window will be set fullscreen for entire desktop which can be set across multiple physical screen.
    DesktopScreen,
}

/// [Window] properties.
pub struct WindowProperty {

    /// Window title
    pub title : String,

    /// Cursor mode and properties
    pub cursor : CursorProperty,

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
    /// Create a new instance of [DisplayProperty] with default values from position and size.
    pub fn new(position : (i32, i32), size : (u32, u32)) -> WindowProperty {
        WindowProperty{ 
            title: String::new(), 
            cursor: CursorProperty::new(), 
            position, 
            size, 
            center: (size.0 as i32 / 2, size.1 as i32 / 2), 
            is_minimized: false, 
            is_maximized: false, 
            is_fullscreen: false 
        }
    }
}

/// Abstraction of a [Window](https://en.wikipedia.org/wiki/Windowing_system#Display_server)
/// and/or [Window manager](https://en.wikipedia.org/wiki/Window_manager) used to create and manage window.
pub trait Window {
    /// Get the window provider managing this window.
    fn get_window_provider(&self) -> WindowProvider;

    /// Pop a window event from the queue.
    fn poll_event(&mut self) -> WindowEvent;

    /// Get the count of events that need handling.
    fn get_event_count(&self) -> usize;

    /// Get windows properties.
    /// 
    /// The [KWindowManager] is responsible for updating this struct.
    fn get_window_property(&self) -> &WindowProperty;

    /// Set the cursor position
    /// 
    /// Must be overridden for desktop implementation.
    fn set_cursor_position(&mut self, position : (i32, i32))  { todo!( )}

    /// Set the cursor mode for the [KWindow] [DisplayEventMouse](enum.DisplayEventMouse.html) events.
    /// 
    /// Must be overridden for desktop implementation.
    fn set_cursor_mode(&mut self, mode : CursorMode)  { todo!( )}

    /// Hide system default cursor.
    /// 
    /// Must be overridden for desktop implementation.
    fn hide_cursor(&mut self)  { todo!( )}

    /// Show system default cursor.
    /// 
    /// Must be overridden for desktop implementation.
    fn show_cursor(&mut self) { todo!( )}

    /// Confine cursor to window, preventing it from exiting boundaries.
    /// 
    /// Must be overridden for desktop implementation.
    fn confine_cursor(&mut self) { todo!( )}

    /// Release cursor from window, allowing it to exit boundaries.
    /// 
    /// Must be overridden for desktop implementation.
    fn release_cursor(&mut self)  { todo!( )}

    /// Restore the window, undoing any minimized, maximized and/or fullscreen status.
    /// 
    /// Must be overridden for desktop implementation.
    fn restore(&mut self)  { todo!( )}

    /// Set a new title for the window.
    fn set_title(&mut self, title : &str);

    /// Set a size for window.
    /// 
    /// Must be overridden for desktop implementation.
    fn set_size(&mut self, size : (u32, u32))  { todo!( )}

     /// Set a position of window.
     /// 
     /// Must be overridden for desktop implementation.
    fn set_position(&mut self, position : (i32, i32))  { todo!( )}

    /// Set the window as fullscreen according to [KFullscreenMode].
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
