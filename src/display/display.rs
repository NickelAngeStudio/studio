/*

use cfg_boost::{ match_cfg, target_cfg};
use crate::error::StudioError;
use crate::error::KWindowError;
use super::KWindowProvider;

use super::KWindowManager;
use super::{event::{ DisplayEvent, DisplayEventDispatcher}, screen::KScreenList};
#[allow(unused_imports)]
use super::event::{ DisplayEventMouse, DisplayEventReceiver };





target_cfg! { 
    desktop => {    // Display implementation for desktop
        use super::cursor::KCursorMode;
        use super::KFullscreenMode;

        /// Minimum [KWindow] width allowed.
        pub const KWINDOW_MIN_WIDTH : u32 = 1;

        /// Minimum [KWindow] height allowed.
        pub const KWINDOW_MIN_HEIGHT : u32 = 1;

        /// Maximum [KWindow] width allowed.
        pub const KWINDOW_MAX_WIDTH : u32 = 65535;

        /// Maximum [KWindow] height allowed.
        pub const KWINDOW_MAX_HEIGHT : u32 = 65535;



    },
    mobile => {     // Display implementation for mobile

    }
}




/// Create and manage a window frame for display. 
/// 
/// [KWindow] broadcasts [DisplayEvent] to multiple [DisplayEventReceiver] via [KWindow::dispatch_events()].
/// 
/// Act as a window factory.
/// 
/// TODO: More doc about OS, dispatch, and Examples
pub struct Display {

    /// KWindow event dispatcher
    dispatcher : DisplayEventDispatcher,

    /// [KWindowManager] that manage the window.
    manager : Box<dyn KWindowManager>,
}
*/
/*
impl KWindow {
    target_cfg! {
        desktop => {
            /// Create a new sized [KWindow] in the middle of the main default screen.
            /// 
            /// Return New [`KWindow`].
            /// 
            /// 
            /// # Error(s)
            /// Returns [StudioError::KWindow(KWindowError::NoDisplayServer)] if no display server found on Linux.
            /// 
            /// Returns [StudioError::KWindow(KWindowError::SizeError)] if width and/or height aren't within allowed boundaries.
            pub fn new(width:u32, height:u32) -> Result<KWindow, StudioError> {

                match_cfg! {
                    linux => super::linux::get_linux_kwindow(width, height),
                    windows => Self::from_provider(KWindowProvider::Windows),
                    _ => todo!()
                }

            }

            /// Create a [KWindow] from specified provider in the middle of primary screen.
            /// 
            /// # Error(s)
            /// Returns [StudioError::KWindow(KWindowError::NotSupported)] if display server not supported.
            /// 
            /// Returns [StudioError::KWindow(KWindowError::SizeError)] if width and/or height aren't within allowed boundaries.
            pub fn from_provider(provider : KWindowProvider, width:u32, height:u32) -> Result<KWindow, StudioError> {   

                if KWindow::is_size_within_boundaries(width, height) {
                    match provider {
                        KWindowProvider::Wayland => todo!(),
                        KWindowProvider::X11 => todo!(),
                        // Anything else is not supported.
                        _ => Err(StudioError::KWindow(KWindowError::NotSupported)),
                    }

                } else {
                    Err(StudioError::KWindow(KWindowError::SizeError))
                }
                
            }

            /// Confine cursor to window, preventing it from exiting boundaries.  
            pub fn confine_cursor(&mut self) {        
                self.manager.confine_cursor();
            }

            /// Get the cursor position with as a pair (x,y).
            pub fn get_cursor_position(&self) -> (i32, i32) {
                self.manager.get_window_property().cursor.position
            }

            /// Get the [KCursorMode] for the [KWindow] [DisplayEventMouse](enum.DisplayEventMouse.html) events.
            pub fn get_cursor_mode(&self) -> KCursorMode{
                self.manager.get_window_property().cursor.mode
            }

            /// Returns position (x,y) of the [KWindow].
            pub fn get_position(&self) -> (i32, i32) {
                self.manager.get_window_property().position
            }

            /// Returns dimension (width, height) of the [KWindow].
            pub fn get_size(&self) -> (u32, u32) {
                self.manager.get_window_property().size
            }

            /// Hide system default cursor.
            pub fn hide_cursor(&mut self) {
                self.manager.hide_cursor();
            }

            /// Get if the cursor is confined to the window, preventing it from going further than window boundaries.
            pub fn is_cursor_confined(&self) -> bool {
                self.manager.get_window_property().cursor.confined
            }

            /// Get if the default operating system cursor is visible.
            pub fn is_cursor_visible(&self) -> bool {
                self.manager.get_window_property().cursor.visible
            }

            /// Returns if the [KWindow] is fullscreen or not.
            pub fn is_fullscreen(&self) -> bool {
                self.manager.get_window_property().is_fullscreen
            }
            

            /// Returns if the [KWindow] is maximized or not.
            pub fn is_maximized(&self) -> bool {
                self.manager.get_window_property().is_maximized
            }

            /// Returns if the [KWindow] is minimized or not.
            pub fn is_minimized(&self) -> bool {
                self.manager.get_window_property().is_minimized
            }

            /// Release cursor from window, allowing it to exit boundaries.
            /// 
            /// Cursor will ALWAYS be released if the window loses focus.
            pub fn release_cursor(&mut self) {
                // Release only if confined.
                if self.manager.get_window_property().cursor.confined {
                    self.release_cursor();
                }
            }

            /// Restore the [KWindow], undoing any minimized, maximized and/or fullscreen status.
            pub fn restore(&mut self) {
                self.restore();
            }

            /// Show system default cursor.
            pub fn show_cursor(&mut self) {
                // Show only if not visible.
                if !self.manager.get_window_property().cursor.visible {
                    self.show_cursor();
                }
            }

            /// Set position of [KWindow] according to position (x,y).
            pub fn set_position(&mut self, position : (i32, i32)){
                self.set_position(position);
            }

            /// Set dimension of [KWindow] according to size (width, height).
            /// 
            /// Returns Ok(0) if successful.
            /// 
            /// # Error(s)
            /// Returns [StudioError::KWindow(KWindowError::SizeError)] if width and/or height not within allowed boundaries.
            pub fn set_size(&mut self, size : (u32, u32)) -> Result<u8, StudioError>{
                // Make sure dimension are within boundaries.
                if KWindow::is_size_within_boundaries(size.0, size.1) {
                    self.set_size(size);
                    Ok(0)
                } else {
                    Err(StudioError::KWindow(KWindowError::SizeError))
                }
            }

            /// Set the [KWindow] as fullscreen according to [KWindowFullscreenMode] parameter.
            pub fn set_fullscreen(&mut self, fs_mode : KFullscreenMode) {
                if !self.manager.get_window_property().is_fullscreen {
                    self.manager.set_fullscreen(fs_mode);
                }
            }

            /// Set the cursor position with a pair (x,y).
            pub fn set_cursor_position(&mut self, position : (i32, i32)){
                self.set_cursor_position(position); 
            }

            /// Set the cursor mode for the [KWindow] [DisplayEventMouse](enum.DisplayEventMouse.html) events.
            pub fn set_cursor_mode(&mut self, mode : KCursorMode) {
                self.manager.set_cursor_mode(mode);
            }

            /// Return True if width and size are between boundaries.
            #[doc(hidden)]
            fn is_size_within_boundaries(width:u32, height:u32) -> bool {

                if width >= KWINDOW_MIN_WIDTH && width <= KWINDOW_MAX_WIDTH && height >= KWINDOW_MIN_HEIGHT && height <= KWINDOW_MAX_HEIGHT {
                    // Withing boundaries
                    true
                } else {
                    // Boundaries overflow
                    false
                }

            }

        },
        mobile => {
            /// Create a new [KWindow] for mobile devices.
            /// 
            /// Return New [`KWindow`] created.
            pub fn new() -> Result<KWindow, StudioError> {        
                todo!()
            }

        }
    }
   
    /// Dispatch [DisplayEvent] to [DisplayEventReceiver] using a [DisplayEventDispatcher].
    /// 
    /// # Note(s)
    /// After dispatching events, [KWindow::sync()] will be called automatically.
    /// 
    /// # Example(s)
    /// Dispatching at each loop call in Main loop
    /// ```
    /// // Create a KWindow
    /// let mut w = KWindow::new(100,100,100,100,true);
    /// 
    /// // Create a dispatcher that doesn't log unhandled events.
    /// let mut ked = DisplayEventDispatcher::new(false);
    /// 
    /// ... add receivers via ked.add_receiver() ...
    /// 
    /// loop {
    ///     // Dispatch events
    ///     w.dispatch_events(&mut ked);
    /// }
    /// ```
    pub fn dispatch_events(&mut self, dispatcher : &mut DisplayEventDispatcher, sync : bool) {

        // First get the event count to poll. This is important to prevent bloking.
        let event_count = self.manager.get_event_count();

        for _ in 0..event_count {
            // Fetch event
            let event = self.manager.poll_event();

            match event {
                DisplayEvent::None => {}, // None event are not dispatched
                _ => {
                    dispatcher.dispatch(&event);
                },
            }
        }

        // Sync events with window manager
        if sync {
            self.manager.sync();
        }
    }

    /// Returns a list of connected screens with details.
    pub fn get_screen_list(&self) -> Result<KScreenList, StudioError> {

        KScreenList::from_provider(self.manager.get_window_provider())

    }

    /// Returns the [KWindow] title. 
    pub fn get_title(&self) -> &str {
        &self.manager.get_window_property().title
    }

    /// Set a new title for the [KWindow].
    pub fn set_title(&mut self, title : &str) {
        self.set_title(title);
    }

    /// Sync all event between client and display server / window manager. 
    /// 
    /// This need to be called at each loop if [KWindow::dispatch_events()] sync parameter = false..
    pub fn sync(&self) {
        self.sync();  
    }
}
*/
/*
/// Private KWindow members
#[doc(hidden)]
impl KWindow {
    

    /// Handle events use for [KWindow] like resizing, closing, etc...
    /// 
    /// Returns true if event was handle and should not be given to receivers.
    #[inline(always)]
    fn handle_kwindow_event(&mut self, event : &DisplayEvent) -> bool {

        match event {
            DisplayEvent::None => true,   // Any event None must not pass
            DisplayEvent::Window(event) => self.handle_kwindow_window_event(event),
            DisplayEvent::Keyboard(event) => self.handle_kwindow_keyboard_event(event),
            DisplayEvent::Mouse(event) => self.handle_kwindow_mouse_event(event),
            DisplayEvent::Controller(event) => self.handle_kwindow_controller_event(event),
            DisplayEvent::Unknown => false,
        }
    }

    /// Handle DisplayEventWindow for KWindow.
    #[meta_cfg(desktop)]
    #[inline(always)]
    fn handle_kwindow_window_event(&mut self, event : &DisplayEventWindow) -> bool {
        
        debug_println!("\x1b[92mDisplayEventWindow::{:?}\x1b[0m", event);
        match event {
            DisplayEventWindow::Moved(position) => {
                self.property.position = *position;
                false
            },
            DisplayEventWindow::Resized(size) => {
                self.property.size = *size;
                self.property.center = (self.property.size.0 as i32 / 2, self.property.size.1 as i32 / 2);
                false
            },
            DisplayEventWindow::MovedResized(position, size) => {
                self.property.position = *position;
                self.property.size = *size;
                self.property.center = (self.property.size.0 as i32 / 2, self.property.size.1 as i32 / 2);
                false
            },
            DisplayEventWindow::CursorEnter() => {
                // Hide cursor if supposed to be hidden.
                if !self.property.cursor.visible {
                    self.manager.hide_cursor();
                }
                false
            },
            DisplayEventWindow::CursorLeave() => {
                // Show hidden cursor when out of window.
                if !self.property.cursor.visible {
                    self.manager.show_cursor();
                }
                false
            },
            DisplayEventWindow::Focus() => {
                // If cursor is confined, confine cursor on focus.
                if self.property.cursor.confined {
                    self.manager.confine_cursor();
                }
                false
            },
            DisplayEventWindow::Blur() => {
                // If cursor is confined, release cursor on blur.
                if self.property.cursor.confined {
                    self.manager.release_cursor();
                }
                false
            },
            DisplayEventWindow::Close() => {
                false
            },

            _ => false,
            
        }
    }

    /// Handle DisplayEventKeyboard for KWindow.
    #[meta_cfg(desktop)]
    #[inline(always)]
    fn handle_kwindow_keyboard_event(&mut self, event : &DisplayEventKeyboard) -> bool {
        debug_println!("\x1b[93mDisplayEventKeyboard::{:?}\x1b[0m", event);
        match event {
            DisplayEventKeyboard::KeyDown(_) => {},
            DisplayEventKeyboard::KeyUp(_) => {},
        }
        false
    }

    /// Handle DisplayEventMouse for KWindow.
    #[meta_cfg(desktop)]
    #[inline(always)]
    fn handle_kwindow_mouse_event(&mut self, event : &DisplayEventMouse) -> bool {
        debug_println!("\x1b[94mDisplayEventMouse::{:?}\x1b[0m", event);
        match event {
            DisplayEventMouse::Moved(position) => match self.property.cursor.mode {
                KCursorMode::Pointer => {
                    // Register cursor position.
                    self.property.cursor.position = *position;
                    false
                },
                KCursorMode::Acceleration => {
                    if *position == (0,0) {     // Ignore position reset
                        true
                    } else { // Reset cursor position
                        self.property.cursor.position = self.property.center;
                        self.set_cursor_position(self.property.center);
                        false
                    }
                }
            },
            _ => false,
        }
    }
    
    /// Handle KWindowController for KWindow.
    #[meta_cfg(desktop)]
    #[inline(always)]
    fn handle_kwindow_controller_event(&mut self, event : &DisplayEventController) -> bool {
        debug_println!("\x1b[95mDisplayEventController::{:?}\x1b[0m", event);
        match event {
            _=> false,
        }
    }
}
*/