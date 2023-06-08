//! Window abstraction and properties

use cfg_boost::{ target_cfg};

use super::manager::WindowManager;
use super::property::WindowProperty;
use super::event::{Event, EventMouse, EventWindow};
use super::screen::{ScreenList, Screen};

// Window type alias definition
target_cfg! {
    linux => {
        /// [Window] type alias with window manager.
        pub type WindowType<'window> = Window<'window, super::provider::linux::LinuxWindowManager>;
    },
}

/// [Window] fullscreen mode enumeration.
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

/// [Window] [show](Window::show()) options.
pub enum WindowShowOption<'window, WM : WindowManager> {

    /// Show the window normally with only root window as parent.
    Normal,

    /// Show the window in fullscreen mode
    Fullscreen(FullScreenMode),

    /// Show the window as child of another parent window. Closing parent window must close all childs.
    Child(&'window mut Window<'window, WM>),

    /// Show the window as a child always on top of parent window. Closing parent window must close all childs.
    Top(&'window mut Window<'window, WM>),

    /// Show window as Modal window. Modal are always on top of parent, blocking parent access until closed.
    Modal(&'window mut Window<'window, WM>),
}

/// Window used to display content. Managed by a window manager.
pub struct Window<'window, WM : WindowManager> {

    /// Manager of window.
    manager : &'window WM,

    /// Properties of window
    properties: WindowProperty<'window, WM>,

    /// Childs of window
    childs: Vec<&'window Window<'window, WM>>,
}

impl<'window, WM> Window<'window, WM> where WM: WindowManager {

    pub fn new() -> Window<'window, WM> {


    }

    /// Get [Window] properties.
    pub fn get_properties(&self) -> &WindowProperty<'window, WM>{
        &self.properties
    }

    /// Get mutable [Window] properties.
    pub fn get_properties_mut(&mut self) -> &WindowProperty<'window, WM>{
        &self.properties
    }

    /// Get list of hardware screen display.
    pub fn get_screen_list(&self) -> &ScreenList {
        &self.manager.get_screen_list()
    }

    /// Pop a window event from the queue.
    pub fn poll_event(&mut self) -> Event{

        let event = self.manager.poll_event();
        match event {
            Event::Window(event) => match event{
                EventWindow::Shown() => todo!(),
                EventWindow::Hidden() => todo!(),
                EventWindow::Exposed(_, _) => todo!(),
                EventWindow::Moved(_) => todo!(),
                EventWindow::MovedResized(_, _) => todo!(),
                EventWindow::Resized(_) => todo!(),
                EventWindow::Minimized() => todo!(),
                EventWindow::Maximized() => todo!(),
                EventWindow::Fullscreen() => todo!(),
                EventWindow::Restored() => todo!(),
                EventWindow::CursorEnter() => todo!(),
                EventWindow::CursorLeave() => todo!(),
                EventWindow::Focus() => todo!(),
                EventWindow::Blur() => todo!(),
                EventWindow::Close() => todo!(),
            },
            Event::Mouse(event) => match event {
                EventMouse::Moved(event) => {
                    // Override cursor according to pointer mode
                },
                _ => event,
            },
            _ => event,
        }

        /*
        match self.pointer.mode {   
            PointerMode::Pointer => {
                self.pointer.position = (xevent._xmotion._x, xevent._xmotion._y);
                Event::Mouse(EventMouse::Moved((xevent._xmotion._x, xevent._xmotion._y)))
            },
            PointerMode::Acceleration => {
                let position = (xevent._xmotion._x - self.property.center.0, 
                    xevent._xmotion._y - self.property.center.1);
                // Report acceleration only if movement occurred
                if position.0 != 0 || position.1 != 0 {
                    // Re-center pointer
                    self.set_pointer_position(self.property.center);

                    // Return position
                    Event::Mouse(EventMouse::Moved(position))
                } else {
                    self.poll_event()
                }
            }
        }

        self.manager.poll_event()
        */
    }

     // Show the window. By default, new windows are hidden and .show() must be called.
    pub fn show(&mut self, option: WindowShowOption<'window, WM>){
        todo!()
    }

    // Hide the window
    pub fn hide(&mut self){
        // Set window property as hidden
        self.properties.visible = false;

        self.manager.hide()
    }
 
    // Force close the window.
    pub fn close(&mut self){
       
       self.manager.close()
    }

    /// Restore the window, undoing any minimized, maximized and/or fullscreen status.
    pub fn restore(&mut self){
        self.manager.restore()
    }

}





/*
/// Abstraction of a [Window](https://en.wikipedia.org/wiki/Windowing_system#Display_server)
/// and/or [Window manager](https://en.wikipedia.org/wiki/Window_manager) used to create and manage window.
pub trait Window {
    /// Get the window provider managing this window.
    fn get_window_provider(&self) -> WindowProvider;

    /// Get the count of events that need handling.
    fn get_event_count(&self) -> usize;

    /// Get [Window] properties.
    fn get_window_properties(&self) -> &WindowProperty;

    /// Get [PointerProperty] for window.
    fn get_pointer_properties(&self) -> &dyn PointerProperty;

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
*/