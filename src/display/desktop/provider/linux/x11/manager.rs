use crate::display::desktop::manager::{WindowManager, WindowManagerParameter};
use crate::display::error::DisplayError;
use crate::error::StudioError;

use std::any::Any;
use std::ptr::{null, null_mut};
use std::thread;
use std::ffi::{ CString, c_int };
use std::{panic::catch_unwind};

use crate::display::desktop::window::{ Window, FullScreenMode, WindowShowOption, WindowChildDisplayOption};
use super::cbind::{attributes::*, constants::*, functs::*, structs::* };

use super::super::super::super::screen::{ScreenList, Screen};
use super::super::super::super::event::{ Event };
use super::super::WindowProvider; 
use super::atom::X11Atoms;
use super::screen::get_x11_screen_list;

/// Pointer left button index for X11
const POINTER_LEFT_BUTTON: u32 = 1;

/// Pointer middle button index for X11
const POINTER_MIDDLE_BUTTON: u32 = 2;

/// Pointer right button index for X11
const POINTER_RIGHT_BUTTON: u32 = 3;

/// Pointer previous button index for X11
const POINTER_PREVIOUS_BUTTON: u32 = 8;

/// Pointer next button index for X11
const POINTER_NEXT_BUTTON: u32 = 9;

/// Pointer scroll up index for X11
const POINTER_SCROLL_UP: u32 = 4;

/// Pointer scroll down index for X11
const POINTER_SCROLL_DOWN: u32 = 5;

/// Pointer scroll left index for X11
const POINTER_SCROLL_LEFT: u32 = 6;

/// Pointer scroll right index for X11
const POINTER_SCROLL_RIGHT: u32 = 7;


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



pub(crate) struct X11WindowManager {

    /// Used to fetch X11 events
    pub(crate) x_event : XEvent,    

    /// Window manager parameters given when showed.
    pub(crate) parameters : WindowManagerParameter,

    /// Retained events that will be sent next poll_event 
    pub(crate) retained_events : Vec<Event>,

    /// C-compatible string for window title
    wm_title : CString,
    
    /// Display connection pointer
    pub(crate) display : *mut X11Display,

    /// Window handle pointer
    pub(crate) window : *mut X11Handle,

    /// Atoms for handling x11 window properties
    pub(crate) atoms : X11Atoms,

    /// Count of event to poll
    pub(crate) event_count : usize,

    /// Window was created
    pub(crate) window_created:bool,
}

impl X11WindowManager {
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

    /// Get the event queue count
    fn get_event_count(&self) -> usize {
        unsafe {
            XEventsQueued(self.display, 0).try_into().unwrap()
        }   
    }

    fn sync(&self) {
        unsafe {
            XSync(self.display, false);
        }
    }


}

/// [Drop] trait implementation for [X11WindowManager].
impl Drop for X11WindowManager {
    fn drop(&mut self) {
        unsafe {
            // Close display server connection.
            XCloseDisplay(self.display);
        }
    }
}

impl<'window> WindowManager<'window> for X11WindowManager {
    fn new() -> Result<Self, StudioError> {
        unsafe{
            let display = XOpenDisplay(std::ptr::null());     // Display connection
            let mut atoms = X11Atoms::new(display);     // X11 Atoms
            
            Ok(X11WindowManager {
                x_event: XEvent{ _type:0 }, 
                retained_events: Vec::new(),
                wm_title: CString::new("").unwrap(), 
                display,
                window: null_mut(),
                atoms,
                event_count: 0,
                parameters: WindowManagerParameter::default(),
                window_created: false,
            })
        }
    }

    fn get_window_provider(&self) -> WindowProvider {
        WindowProvider::X11
    }

    #[inline(always)]
    fn poll_event(&mut self) -> Event  {
        // Get count to poll
        if self.event_count == 0 {
            self.sync();
            self.event_count = self.get_event_count();
        }
        self.get_event()
    }

    #[inline(always)]
    fn send_event(&mut self, event : Event) {
        self.retained_events.push(event);   // Keep event as retained to be poll next.
        self.event_count += 1;
    }

    #[inline(always)]
    fn show(&mut self, parameters : WindowManagerParameter) {
        self.parameters = parameters;


    }

    #[inline(always)]
    fn show_child(&mut self, parent : &dyn WindowManager, parameters : WindowManagerParameter, option : WindowChildDisplayOption) {
        
        // Downcast parent to X11WindowManager.
        match parent.as_any().downcast_ref::<X11WindowManager>(){
            Some(parent) => {

            },
            _ => panic!("Downcast failed!"),    // Shoudln't happen.
        }

    }

    

    #[inline(always)]
    fn restore(&mut self) {
        todo!()
    }

    #[inline(always)]
    fn close(&mut self) {
        todo!()
    }

    #[inline(always)]
    fn hide(&mut self) {
        todo!()
    }

    #[inline(always)]
    fn get_screen_list(&self) -> Result<&ScreenList, StudioError> {
        
        let screens = get_x11_screen_list();

        match screens{
            Ok(screens) => Ok(&screens),
            Err(err) => Err(err),
        }
    }

    #[inline(always)]
    fn show_decoration(&mut self) {
        if !self.parameters.decoration {
            unsafe {
                XChangeProperty(self.display, self.window, self.atoms._MOTIF_WM_HINTS, self.atoms._MOTIF_WM_HINTS,
                    32, PropModeReplace, std::mem::transmute(&[2, 0, 0, 0, 0]), 5);

                //x11_change_property!(self.display, self.window, self.atoms, _NET_WM_STATE, _NET_WM_STATE_MAXIMIZED_HORZ, _NET_WM_STATE_MAXIMIZED_VERT, _NET_WM_STATE_FULLSCREEN);
            }
            self.parameters.decoration = true;
        }
    }

    #[inline(always)]
    fn hide_decoration(&mut self) {
        if self.parameters.decoration {
            unsafe {
                XChangeProperty(self.display, self.window, self.atoms._MOTIF_WM_HINTS, self.atoms._MOTIF_WM_HINTS,
                    32, PropModeReplace, std::mem::transmute(&[2, 0, 0, 0, 0]), 5);

                //x11_change_property!(self.display, self.window, self.atoms, _NET_WM_STATE, _NET_WM_STATE_MAXIMIZED_HORZ, _NET_WM_STATE_MAXIMIZED_VERT, _NET_WM_STATE_FULLSCREEN);
            }

            self.parameters.decoration = false;
        }
    } 

    #[inline(always)]
    fn set_title(&mut self, title:&str) {
        unsafe {
            self.wm_title = CString::from_vec_unchecked(title.as_bytes().to_vec());
            XStoreName(self.display, self.window, self.wm_title.as_ptr() as *mut i8);
        }
    }

    #[inline(always)]
    fn set_size(&mut self, size : (u32, u32)) {
        unsafe {
            // Keep real window position
            let position = X11WindowManager::get_x11_window_position(self.display, self.window);

            XResizeWindow(self.display, self.window, size.0, size.1);
            
            // Reposition window since resize put it back at 0,0
            self.set_position(position);
        }
    }

    #[inline(always)]
    fn set_position(&mut self, position : (i32,i32)) {
        unsafe {
            XMoveWindow(self.display, self.window, position.0, position.1);
        }
    }

    #[inline(always)]
    fn set_pointer_position(&mut self, position : (i32,i32)) {
        unsafe {
            XWarpPointer(self.display, self.window, self.window, 0, 0, 
                0, 0, position.0,  position.1);
        }
    }

    #[inline(always)]
    fn hide_pointer(&mut self) {
        unsafe {
            if self.parameters.pointer_visible {
                self.parameters.pointer_visible = false;
                XFixesHideCursor(self.display, self.window);
            }
        }
    }

    #[inline(always)]
    fn show_pointer(&mut self) {
        unsafe {
            if !self.parameters.pointer_visible {    // Make sure X hide cursor was called prior to show.
                XFixesShowCursor(self.display, self.window);
                self.parameters.pointer_visible = true;
            }       
        }
    }

    #[inline(always)]
    fn confine_pointer(&mut self) {
        unsafe {
            self.parameters.pointer_confined = true;
            XGrabPointer(self.display, self.window, true, 
            0, GrabModeAsync.try_into().unwrap(), GrabModeAsync.try_into().unwrap(), self.window, 0, CurrentTime);
        }
    }

    #[inline(always)]
    fn release_pointer(&mut self) {
        unsafe {
            self.parameters.pointer_confined  = false;
            XUngrabPointer(self.display, CurrentTime);
        }
    }

    #[inline(always)]
    fn enable_autorepeat(&mut self) {
        self.parameters.auto_repeat = true;
    }

    #[inline(always)]
    fn disable_autorepeat(&mut self) {
        self.parameters.auto_repeat = false;
    }

    #[inline(always)]
    fn get_left_button_index(&self) -> u32 {
        POINTER_LEFT_BUTTON
    }

    #[inline(always)]
    fn get_right_button_index(&self) -> u32 {
        POINTER_RIGHT_BUTTON
    }

    #[inline(always)]
    fn get_middle_button_index(&self) -> u32 {
        POINTER_MIDDLE_BUTTON
    }

    #[inline(always)]
    fn get_next_button_index(&self) -> u32 {
        POINTER_NEXT_BUTTON
    }

    #[inline(always)]
    fn get_previous_button_index(&self) -> u32 {
        POINTER_PREVIOUS_BUTTON
    }

    #[inline(always)]
    fn get_scroll_up_index(&self) -> u32 {
        POINTER_SCROLL_UP
    }

    #[inline(always)]
    fn get_scroll_down_index(&self) -> u32 {
        POINTER_SCROLL_DOWN
    }

    #[inline(always)]
    fn get_scroll_left_index(&self) -> u32 {
        POINTER_SCROLL_LEFT
    }

    #[inline(always)]
    fn get_scroll_right_index(&self) -> u32 {
        POINTER_SCROLL_RIGHT
    }

    fn as_any(&'window self) -> &dyn Any {
        &self
    }

      
}

impl X11WindowManager {
    /// Create x11 Window to be showed.
    #[inline(always)]
    pub fn create_window(&self, display : *mut X11Display, root : *mut X11Handle) -> *mut X11Handle {
        unsafe {
            let window = XCreateSimpleWindow(display, root, self.parameters.position.0,self.parameters.position.1,
                self.parameters.size.0, self.parameters.size.1, 0, 0, 0);

            // Set window Type to normal
            x11_change_property!(display, window, self.atoms, _NET_WM_WINDOW_TYPE, _NET_WM_WINDOW_TYPE_NORMAL);

            // Set window protocols to capture window closing
            XSetWMProtocols(display, window, &mut self.atoms.WM_DELETE_WINDOW, 1);

            // Allowed actions
            x11_change_property!(display, window, self.atoms, _NET_WM_ALLOWED_ACTIONS, _NET_WM_ACTION_FULLSCREEN, _NET_WM_ACTION_MINIMIZE, _NET_WM_ACTION_CHANGE_DESKTOP,
                _NET_WM_ACTION_CLOSE, _NET_WM_ACTION_ABOVE, _NET_WM_ACTION_BELOW);

            if self.parameters.full_screen {
                // Set as fullscreen
                 x11_change_property!(display, window, self.atoms, _NET_WM_STATE, _NET_WM_STATE_MAXIMIZED_HORZ, _NET_WM_STATE_MAXIMIZED_VERT, _NET_WM_STATE_FULLSCREEN);
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