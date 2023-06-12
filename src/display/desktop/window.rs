//! Window abstraction and properties


use std::cell::RefCell;
use std::rc::Rc;

use cfg_boost::{ target_cfg};

use crate::error::StudioError;


use super::property::{WindowPropertySet, WindowProperty};
use super::event::{Event};
use super::provider::WindowProvider;
use super::screen::{ScreenList};

// Window type alias definition
target_cfg! {
    linux => {
        /// [Window] type alias according to platform.
        pub type WindowType = super::provider::linux::LinuxWindow;
    },
}



/// Window abstraction.
/// 
/// Steps :
/// new()
/// set_properties
/// show()
pub trait Window {

    // Create a new [Window] wrapped in a mutable reference counter.
    /// 
    /// Returns Ok([Window]) on success, Err([StudioError]) on error.
    fn new() -> Result<Self, StudioError> where Self: Sized;

    /// Show the window. By default, new windows are hidden and .show() must be called.
    /// 
    /// Return Ok(true) on success, Err(StudioError) on failure.
    fn show(&mut self) -> Result<bool, StudioError>;

    // Hide the window. The window ressources are not freed and can still receive events.
    fn hide(&mut self);
 
    // Force close the window. The window ressources are freed and cannot receive events.
    fn close(&mut self);

    /// Pop a window event from the queue.
    fn poll_event(&mut self) -> Event;
    
    /// Get the provider id of this window
    fn get_provider(&self) -> WindowProvider;

    /// Get window properties in a read only struct.
    fn get_properties(&self) -> &WindowProperty;

    /// Set a window property according to family.
    /// 
    /// Return Ok() with the count of property changed on success, Err(StudioError) on failure with the property that cause the failure.
    fn set_property(&mut self, property : WindowPropertySet) -> Result<usize, (WindowPropertySet, StudioError)>;

    /// Set multiple window properties from an array. When setting multiple properties, this function 
    /// is faster because some property need to recreate window. Here this will be done only one time
    /// if window recreate is needed.
    /// 
    /// Returns Ok() the count of properties changed on success, Err(WindowPropertySet, StudioError) on failure with the
    /// property that cause the failure.
    fn set_properties(&mut self, properties : &[WindowPropertySet]) -> Result<usize, (WindowPropertySet, StudioError)>;

    /// Get the OS Window manager window handle.
    fn get_window_handle(&self) -> Option<*const usize>;

    target_cfg! {
        linux => {
            /// Get the OS Window manager display handle.
            fn get_display_handle(&self) -> Option<*const usize>;
        }
    }

}

/*
/// Window used to display content. Managed by a window manager.
pub struct Window<'window> {

    /// Manager of window.
    manager : WindowManagerType,

    /// Properties of window
    properties: WindowProperty<'window>,

    /// Parent of the window (if any)
    parent : Option<&'window Window<'window>>,

    /// Childs of window
    childs: Vec<WindowChild<'window>>,
}

impl<'window> Drop for Window<'window>{
    fn drop(&mut self) {
        // Force close each child
        self.childs.iter_mut().for_each(|c| c.window.close() );

        // Clear child list
        self.childs.clear();
    }
}

impl<'window> Window<'window> {

    /// Create a new [Window] with it's [WindowManager].
    /// 
    /// Returns Ok([Window]) on success, Err([StudioError]) on error.
    pub fn new() -> Result<Window<'window>, StudioError> {

        match WindowManagerType::new() {
            Ok(mut manager) => {
                let properties = WindowProperty::new(&mut manager);
                Ok(Window { manager, properties, childs: Vec::new(), parent: None })
            },
            Err(err) => Err(err),
        }
        
    }

    /// Get the [WindowManager] managing this window.
    pub fn get_manager(&self) -> &WindowManagerType {
        &self.manager
    }

    /// Get [Window] properties.
    pub fn get_properties(&self) -> &WindowProperty<'window>{
        &self.properties
    }

    /// Get mutable [Window] properties.
    pub fn get_properties_mut(&mut self) -> &WindowProperty<'window>{
        &self.properties
    }

    /// Get list of hardware screen display.
    pub fn get_screen_list(&self) -> Result<&ScreenList, StudioError> {
        self.manager.get_screen_list()
    }

    /// Pop a window event from the queue.
    pub fn poll_event(&mut self) -> Event{

        let event = self.manager.poll_event();
        match event {
            Event::Window(w_event) => self.handle_window_event(&event, &w_event),
            Event::Mouse(m_event) => self.handle_mouse_event(&event, &m_event),
            _ => event,
        }       
    }

    /// Show the window. By default, new windows are hidden and .show() must be called.
    ///
    /// Returns Ok(true) if successfull or Err(StudioError) if failed.
    /// 
    /// Possible Errors :
    /// [DisplayError::ChildAlreadyOwned] happens when a child is already added or owned by another window,
    pub fn show(&'window mut self, option: WindowShowOption<'window>) -> Result<bool,StudioError>{
        let mut parameters = WindowManagerParameter::from_property(&self.properties);

        match option {
            WindowShowOption::Normal => {
                parameters.full_screen = false;
                self.manager.show(parameters);
            },
            WindowShowOption::Fullscreen(fsmode) => {
                match self.set_fullscreen_parameters(&parameters, fsmode){
                    Ok(_) => {},
                    Err(err) => return Err(err),
                }
                self.manager.show(parameters);
            },
            WindowShowOption::Child(parent, option) => {
                parameters.full_screen = false;
                parent.add_child(self, option);

                parent.manager.show_child(parent.get_manager(), parameters, option);
            },
        }

        Ok(true)

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

    /// Handle [EventWindow] before sending it to client.
    #[inline(always)]
    fn handle_window_event(&mut self, event : &Event, w_event : &EventWindow) -> Event {
        match w_event {
            EventWindow::Closed() => {  // Window has been closed.
                match self.parent {
                    Some(parent) => {
                        // Remove child from parent list
                        parent.remove_child(self);
                        self.parent=None;
                    },
                    None => todo!(),
                }
            }
            EventWindow::Shown() => self.properties.visible = false,
            EventWindow::Hidden() => self.properties.visible = true,
            EventWindow::Moved(position) => self.properties.position = *position,
            EventWindow::MovedResized(position, size) => {
                self.properties.position = *position;
                self.properties.size = *size;
            },
            EventWindow::Resized(size) => self.properties.size = *size,
            EventWindow::Minimized() => {
                self.properties.minimized = true;
                self.properties.maximized = false;
                self.properties.fullscreen = false;

            },
            EventWindow::Maximized() => {
                self.properties.minimized = false;
                self.properties.maximized = true;
                self.properties.fullscreen = false;
            },
            EventWindow::Fullscreen() => {
                self.properties.minimized = false;
                self.properties.maximized = true;
                self.properties.fullscreen = true;
            },
            EventWindow::Restored() => {
                self.properties.minimized = false;
                self.properties.maximized = false;
                self.properties.fullscreen = false;
            },
            _ => {},
        }

        *event
    }

    /// Handle [EventMouse] before sending it to client.
    #[inline(always)]
    fn handle_mouse_event(&mut self, event : &Event, m_event : &EventMouse) -> Event {
        match m_event {
            EventMouse::Moved(position) => {
                // Override cursor according to pointer mode
                match self.properties.pointer.mode {
                    PointerMode::Cursor => *event,   // Send event as is.
                    PointerMode::Acceleration => {
                        // Calc delta acceleration
                        let position = (position.0 - self.properties.center.0, 
                            position.1 - self.properties.center.1);

                            if position.0 != 0 && position.1 != 0 { // Send acceleration only if it moved.
                                // Reset pointer to center
                                self.manager.set_pointer_position(self.properties.center);

                                // Send acceleration event.
                                Event::Mouse(EventMouse::Acceleration(position))
                            } else {
                                self.poll_event()   // Ignore and poll next event
                            }
                        },     
                }
            },
            _ => *event,
        }
    }

    /// Add a child to the window.
    /// 
    /// Returns Ok(true) on success or Err(StudioError) if already added or owned.
    fn add_child(&'window mut self, child: &'window mut Window<'window>, format: WindowChildDisplayOption) -> Result<bool, StudioError>{

        match child.parent {
            Some(_) => {    // Child is already owned, send error
                Err(StudioError::Display(crate::display::error::DisplayError::ChildAlreadyOwned))
            },
            None => {
                child.parent = Some(self);
                
                self.childs.push(WindowChild { window : child, format });

                Ok(true)
            },
        } 
    }

    /// Remove a child from the childs array.
    /// 
    /// Return Ok(index) on success, Err(NotFound) on failure
    fn remove_child(&mut self, child: &'window mut Window<'window>)-> Result<usize, StudioError>{
        match self.get_child_index(child) {
            Ok(index) => {
                self.childs.remove(index);
                Ok(index)
            },
            Err(err) => Err(err),
        }
    }

    /// Get index of child in array.
    /// 
    /// Returns Ok(index) on success, NotFound on failure.
    fn get_child_index(&mut self, window: &'window Window<'window>) -> Result<usize, StudioError> {
        for i in 0..self.childs.len() {
            match self.childs.get(i){
                Some(c) => if c.window as *const _ == window as *const _ {
                    return Ok(i)
                },
                None => {},
            }
        }

        Err(StudioError::Display(crate::display::error::DisplayError::ChildNotFound))
    }

    /// Set fullscreen parameters according to full screen mode.
    /// 
    /// Returns Ok(true) on success, Err(StudioError) on failure
    #[inline(always)]
    fn set_fullscreen_parameters(&self, parameters : &WindowManagerParameter, fsmode : FullScreenMode) -> Result<bool,StudioError> {
        parameters.full_screen = true;

        match fsmode {
            FullScreenMode::Primary => {
                match self.manager.get_screen_list() {
                    Ok(screens) => {
                        match screens.get_primary_screen(){
                            Some(screen) => {
                                parameters.position=(screen.get_extended_position().0, screen.get_extended_position().1);
                            },
                            None => {
                                return Err(StudioError::Display(crate::display::error::DisplayError::ScreenDetailError));
                            },
                        }
                    },
                    Err(err) => return Err(err),   // Return error
                }
            },
            FullScreenMode::Desktop => {
                match self.manager.get_screen_list() {
                    Ok(screens) => {
                        parameters.position=(0,0);
                        parameters.size = (screens.get_desktop_width(),screens.get_desktop_height());
                    },
                    Err(err) => return Err(err),
                }
            },
            FullScreenMode::Screen(screen) => {
                // Put window position in screen for ownedship
                parameters.position=(screen.get_extended_position().0, screen.get_extended_position().1);
            },
            FullScreenMode::Current => {},  // Nothing to add here
        }

        Ok(true)

    }
}
*/
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