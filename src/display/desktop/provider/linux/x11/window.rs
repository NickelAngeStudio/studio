use std::thread;
use std::ffi::{ CString, c_int };
use std::{panic::catch_unwind};

use crate::display::desktop::keyboard::KeyboardProperty;
use crate::display::desktop::pointer::{PointerMode, PointerProperty};
use crate::display::desktop::window::{ Window, FullscreenMode, WindowProperty};
use crate::error::StudioError;
use super::cbind::{attributes::*, constants::*, functs::*, structs::* };

use super::super::super::super::screen::{ScreenList, Screen};
use super::super::super::super::event::{ Event };
use super::super::WindowProvider; 
use super::atom::X11Atoms;

/// Pointer left button index for X11
const POINTER_LEFT_BUTTON: u8 = 1;

/// Pointer middle button index for X11
const POINTER_MIDDLE_BUTTON: u8 = 2;

/// Pointer right button index for X11
const POINTER_RIGHT_BUTTON: u8 = 3;

/// Pointer previous button index for X11
const POINTER_PREVIOUS_BUTTON: u8 = 8;

/// Pointer next button index for X11
const POINTER_NEXT_BUTTON: u8 = 9;

/// Pointer scroll up index for X11
const POINTER_SCROLL_UP: u8 = 4;

/// Pointer scroll down index for X11
const POINTER_SCROLL_DOWN: u8 = 5;

/// Pointer scroll left index for X11
const POINTER_SCROLL_LEFT: u8 = 6;

/// Pointer scroll right index for X11
const POINTER_SCROLL_RIGHT: u8 = 7;



/*
const EVENT_MASK : i64 = NoEventMask | KeyPressMask |KeyReleaseMask|ButtonPressMask|ButtonReleaseMask|EnterWindowMask|LeaveWindowMask|PointerMotionMask|
PointerMotionHintMask|Button1MotionMask|Button2MotionMask|Button3MotionMask|Button4MotionMask|Button5MotionMask|ButtonMotionMask|KeymapStateMask|
ExposureMask|VisibilityChangeMask|StructureNotifyMask|ResizeRedirectMask|SubstructureNotifyMask|SubstructureRedirectMask|FocusChangeMask|PropertyChangeMask|
ColormapChangeMask|OwnerGrabButtonMask;
*/

/// Event mask used with x11 to capture and dispatch event.
const EVENT_MASK : i64 =    KeyPressMask | KeyReleaseMask |             // Keyboard Button Down and Up
                            ButtonPressMask | ButtonReleaseMask |       // Controller button??? TBD 
                            EnterWindowMask | LeaveWindowMask |         // Window focus, blur
                            PointerMotionMask | Button1MotionMask | 
                            Button2MotionMask | Button3MotionMask |
                            Button4MotionMask | Button5MotionMask |
                            ButtonMotionMask |                          // Mouse motion??? TBD
                            StructureNotifyMask |                       // ResizeRedirectMask |
                            VisibilityChangeMask | FocusChangeMask |
                            PropertyChangeMask | ExposureMask;          // Window event I guess??
                            



/// Shortcut macro used to change x11 atoms properties
macro_rules! x11_change_property {
    ($display:expr, $window:expr, $x11_property:expr, $property:ident $(,$atoms:ident)+) => {

        // Put atoms in 1 array.
        let atoms_arr = [$($x11_property.$atoms,)+];

        // Push properties change
        XChangeProperty($display, $window, $x11_property.$property,
            $x11_property.xa_atom, 32, PropModeReplace, std::mem::transmute(&atoms_arr), atoms_arr.len() as i32);
    }
}

/// [X Window System](https://en.wikipedia.org/wiki/X_Window_System) implementation of [Window].
pub struct X11Window {

    /// Used to fetch X11 events
    pub(crate) x_event : XEvent,    

    /// Retained event that will be sent next poll_event 
    pub(crate) retained_event : Option<Event>,

    /// C-compatible string for window title
    pub(crate) wm_title : CString,

    /// Display connection pointer
    pub(crate) display : *mut X11Display,

    /// Window handle pointer
    pub(crate) window : *mut X11Handle,

    /// Window properties
    pub(crate) property : WindowProperty,
 
    /// Pointer properties
    pub(crate) pointer : PointerProperty,

    /// Keyboard properties
    pub(crate) keyboard : KeyboardProperty,

    /// Atoms for handling x11 window properties
    pub(crate) atoms : X11Atoms,

    /// Position and size for restoring window.
    pub(crate) restoration_position_size : ((i32,i32),(u32,u32)),

    /// Screen list
    pub(crate) screens : ScreenList,

    /// Count of event to poll
    pub(crate) event_count : usize,
}

/// Public members of [X11Window].
impl X11Window {
    /// Create a new instance of X11Window.
    pub(crate) fn new(width:u32, height:u32) -> X11Window {
        unsafe {
            let screens = ScreenList::from_provider(WindowProvider::X11).unwrap();
            let display = XOpenDisplay(std::ptr::null());     // Display connection
            let mut atoms = X11Atoms::new(display);     // X11 Atoms
            let position = Self::start_position(screens.get_primary_screen().unwrap(), (width, height));    // Window startup position
            let root = Self::get_x11_default_root_window(display);  // Root window
            let property = WindowProperty::new(position, (width, height));

            X11Window{ 
                x_event: XEvent{ _type:0 }, 
                wm_title: CString::new("").unwrap(), 
                display, //: XOpenDisplay(std::ptr::null()), 
                window: Self::create_x11_window(display, root, &mut atoms, position, (width, height), false), 
                atoms,
                restoration_position_size: (position, (width, height)),
                screens,
                event_count: 0,
                pointer: PointerProperty::new(POINTER_LEFT_BUTTON,POINTER_RIGHT_BUTTON,POINTER_MIDDLE_BUTTON,POINTER_NEXT_BUTTON,
                    POINTER_PREVIOUS_BUTTON, POINTER_SCROLL_UP, POINTER_SCROLL_DOWN, POINTER_SCROLL_LEFT, POINTER_SCROLL_RIGHT),
                property,
                retained_event: Option::None,  // No retained event for now.
                keyboard: KeyboardProperty::new(),  
            }
        }
    }

    

    /// Return True if X Window System is supported. False otherwise.
    /// 
    /// Test is done in another thread to prevent main thread panic.
    pub fn is_supported() -> bool {
        unsafe {
            let thread_join_handle = thread::spawn(move || {
                // Try to call C function with error handling.
                let result = catch_unwind(|| {
                    XOpenDisplay(std::ptr::null())
                }); 

                match result {
                    Ok(display) => {
                        if display == std::ptr::null_mut() {
                            false
                        } else {
                            // Disconnect display before returning true
                            XCloseDisplay(display);

                            true
                        }
                    },

                    // Error occurred, not compatible.
                    Err(_) => false,
                }
            });

            match thread_join_handle.join() {
                Ok(value) => value,
                Err(_) => {
                    // Not supported
                    false
                },
            }
        }
    }

    /// Get the X system display connection.
    pub fn get_display_server_connection(&self) -> *const X11Display {
        self.display
    }

    /// Get the X system window handle.
    pub fn get_window_handle(&self) -> *const X11Handle {
        self.window
    }
}

/// [Drop] trait implementation for [X11Window].
impl Drop for X11Window {
    fn drop(&mut self) {
        unsafe {
            // Close display server connection.
            XCloseDisplay(self.display);
        }
    }
}

/// [Window] trait implementation for [X11Window].
impl Window for X11Window {
    fn get_window_provider(&self) -> WindowProvider {
        WindowProvider::X11
    }

    fn set_pointer_mode(&mut self, mode : PointerMode)  {
         self.pointer.mode = mode;

         match mode {
            // Set cursor to center if Acceleration
            PointerMode::Acceleration => self.set_pointer_position(self.property.center),
            _ => {},
        }
    }

    fn get_event_count(&self) -> usize {
        unsafe {
            XEventsQueued(self.display, 0).try_into().unwrap()
        }   
    }

    fn set_pointer_position(&mut self, position : (i32, i32)) {
        unsafe {
            self.pointer.position = position;
            XWarpPointer(self.display, self.window, self.window, 0, 0, 
                0, 0, position.0,  position.1);
        }
    }

    fn hide_pointer(&mut self) {
        unsafe {
            if self.pointer.is_visible {
                self.pointer.is_visible = false;
                XFixesHideCursor(self.display, self.window);
            }
        }
    }

    fn show_pointer(&mut self)  {
        unsafe {
            if !self.pointer.is_visible {    // Make sure X hide cursor was called prior to show.
                XFixesShowCursor(self.display, self.window);
                self.pointer.is_visible = true;
            }       
        }
    }

    fn confine_pointer(&mut self) {
        unsafe {
            self.pointer.is_confined = true;
            XGrabPointer(self.display, self.window, true, 
            0, GrabModeAsync.try_into().unwrap(), GrabModeAsync.try_into().unwrap(), self.window, 0, CurrentTime);
        }
    }

    fn release_pointer(&mut self) {
        unsafe {
            self.pointer.is_confined = false;
            XUngrabPointer(self.display, CurrentTime);
        }
    }

    fn restore(&mut self) {
        unsafe {
            // Destroy current window
            XDestroyWindow(self.display, self.window);

            // Recreate window as normal
            self.window = X11Window::create_x11_window(self.display, XDefaultRootWindow(self.display), &mut self.atoms, self.restoration_position_size.0,
                self.restoration_position_size.1, false);   

            self.set_position(self.restoration_position_size.0);
        }        
    }

    fn set_title(&mut self, title : &str) {
        unsafe {
            self.property.title = String::from(title);
            self.wm_title = CString::from_vec_unchecked(title.as_bytes().to_vec());
            XStoreName(self.display, self.window, self.wm_title.as_ptr() as *mut i8);
        }
    }

    fn set_size(&mut self, size : (u32, u32)) -> Result<(u32, u32), StudioError> {
        if WindowProperty::is_size_within_boundaries(size){
            unsafe {
                // Keep real window position
                self.restoration_position_size.0 = X11Window::get_x11_window_position(self.display, self.window);

                XResizeWindow(self.display, self.window, size.0, size.1);
                
                // Reposition window since resize put it back at 0,0
                self.set_position(self.restoration_position_size.0);

                Ok(size)
            }
        } else {
            Err(StudioError::Display(crate::display::error::DisplayError::SizeError))
        }
    }

    fn set_position(&mut self, position : (i32, i32)) {
        unsafe {
            XMoveWindow(self.display, self.window, position.0, position.1);
        }
    }

    fn set_fullscreen(&mut self, fs_mode : FullscreenMode) {
        unsafe {

            if !self.property.is_fullscreen {
                // Save windowed properties for restoration.
                self.restoration_position_size = (X11Window::get_x11_window_position(self.display, self.window), self.property.size);
            }

            // Destroy current window
            XDestroyWindow(self.display, self.window);

            match fs_mode {
                FullscreenMode::CurrentScreen => {
                    // Recreate window as fullscreen
                    self.window = X11Window::create_x11_window(self.display, XDefaultRootWindow(self.display),
                     &mut self.atoms, (0,0),   self.screens.get_primary_screen().unwrap().get_current_resolution(), true);      
                },
                FullscreenMode::PrimaryScreen => {
                    todo!()
                },
                FullscreenMode::DesktopScreen => {
                    todo!()
                },
            }

            // Write stored title
            XStoreName(self.display, self.window, self.wm_title.as_ptr() as *mut i8);
        }
    }

    fn sync(&self) {
        unsafe {
            XSync(self.display, false);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    #[allow(non_upper_case_globals)]
    fn poll_event(&mut self) -> Event {
        // Get count to poll
        if self.event_count == 0 {
            self.sync();
            self.event_count = self.get_event_count();
        }
        self.get_event()
    }

    fn get_window_properties(&self) -> &crate::display::desktop::window::WindowProperty {
        &self.property
    }

    fn get_pointer_properties(&self) -> &PointerProperty {
        &self.pointer
    }

    fn get_keyboard_properties(&self) -> &crate::display::desktop::keyboard::KeyboardProperty {
        &self.keyboard
    }

    fn enable_autorepeat(&mut self) {
        self.keyboard.auto_repeat = true;
    }

    fn disable_autorepeat(&mut self) {
        self.keyboard.auto_repeat = false;
    }


    

    
}


/// Private members of [X11Window].
impl X11Window {
    /// Create x11 Window according to position, size and if fullscreen or not.
    #[inline(always)]
    pub fn create_x11_window(display : *mut X11Display, root : *mut X11Handle, atoms : &mut X11Atoms, position : (i32, i32), 
        size : (u32,u32), fullscreen : bool) -> *mut X11Handle {
        unsafe {
            let window = XCreateSimpleWindow(display, root, position.0,position.1,
                    size.0, size.1, 4, 0, 0);

            // Set window Type to normal
            x11_change_property!(display, window, atoms, _NET_WM_WINDOW_TYPE, _NET_WM_WINDOW_TYPE_NORMAL);

            // Set window protocols to capture window closing
            XSetWMProtocols(display, window, &mut atoms.WM_DELETE_WINDOW, 1);

            // Allowed actions
            x11_change_property!(display, window, atoms, _NET_WM_ALLOWED_ACTIONS, _NET_WM_ACTION_FULLSCREEN, _NET_WM_ACTION_MINIMIZE, _NET_WM_ACTION_CHANGE_DESKTOP,
                _NET_WM_ACTION_CLOSE, _NET_WM_ACTION_ABOVE, _NET_WM_ACTION_BELOW);

            if fullscreen {
                // Set as fullscreen
                 x11_change_property!(display, window, atoms, _NET_WM_STATE, _NET_WM_STATE_MAXIMIZED_HORZ, _NET_WM_STATE_MAXIMIZED_VERT, _NET_WM_STATE_FULLSCREEN);
            }

            // Map window to display
            XMapWindow(display, window);

            // Mask of events to receive
            XSelectInput(display, window, EVENT_MASK);

            // Flush buffer
            XFlush(display);
            
            // Return window pointer
            window
        }
    }


    /// Generate window start position from default screen and size.
    fn start_position(screen : &Screen, size : (u32, u32)) -> (i32, i32) {

        ((screen.get_current_resolution().0 as i32) / 2 - (size.0 as i32) / 2, (screen.get_current_resolution().1 as i32) / 2 - (size.1 as i32) / 2)

    }

    /// Get default root window of display
    fn get_x11_default_root_window(display : *mut X11Handle) -> *mut X11Handle {
        unsafe {
            XDefaultRootWindow(display)
        }
    }


    /// Get the real, translated position of Display.
    /// 
    /// Reference(s)
    /// <https://stackoverflow.com/questions/3806872/window-position-in-xlib>
    pub fn get_x11_window_position(display : *mut X11Display, window: *mut X11Handle) -> (i32, i32){
        unsafe {
            let mut x : c_int = 0;
            let mut y : c_int = 0;
            let mut child : X11Handle = 0;
            
            XTranslateCoordinates(display, window, 
                XDefaultRootWindow(display), 0, 0, &mut x, &mut y, &mut child );
            let xwa = Self::get_x11_window_attributes(display, window);
            (x - xwa.x, y - xwa.y )
        }
    }

    /// Get the XWindowAttributes from display connection and window handle.
    fn get_x11_window_attributes(display : *mut X11Display, window: *mut X11Handle) -> XWindowAttributes {
        unsafe {
            let mut xwa = XWindowAttributes::empty();
            XGetWindowAttributes( display, window, &mut xwa );
            xwa
        }
    }
   
}