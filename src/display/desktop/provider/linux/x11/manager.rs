use crate::display::desktop::manager::{WindowManager, WindowManagerShowParameters};
use crate::display::error::DisplayError;
use crate::error::StudioError;

use std::ptr::{null, null_mut};
use std::thread;
use std::ffi::{ CString, c_int };
use std::{panic::catch_unwind};

use crate::display::desktop::window::{ Window, FullScreenMode, WindowShowOption};
use super::cbind::{attributes::*, constants::*, functs::*, structs::* };

use super::super::super::super::screen::{ScreenList, Screen};
use super::super::super::super::event::{ Event };
use super::super::WindowProvider; 
use super::atom::X11Atoms;
use super::screen::get_x11_screen_list;

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

    /// Retained events that will be sent next poll_event 
    pub(crate) retained_events : Vec<Event>,

    /// C-compatible string for window title
    pub(crate) wm_title : CString,
    
    /// Display connection pointer
    pub(crate) display : *mut X11Display,

    /// Window handle pointer
    pub(crate) window : *mut X11Handle,

    /// List of harware screen display
    pub(crate) screens : ScreenList,

    /// Atoms for handling x11 window properties
    pub(crate) atoms : X11Atoms,

    /// Count of event to poll
    pub(crate) event_count : usize,

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

impl WindowManager for X11WindowManager {
    fn new() -> Result<Self, StudioError> {
        unsafe{
            let display = XOpenDisplay(std::ptr::null());     // Display connection
            let mut atoms = X11Atoms::new(display);     // X11 Atoms
            let screens = get_x11_screen_list();

            match screens{
                // Return new window manager wrapped in OK
                Ok(screens) => Ok(X11WindowManager {
                    x_event: XEvent{ _type:0 }, 
                    retained_events: todo!(),
                    wm_title: CString::new("").unwrap(), 
                    display,
                    window: null_mut(),
                    atoms,
                    event_count: 0,
                    screens,
                }),

                // Return error
                Err(err) => Err(err),
            }

            
        }
    }

    #[inline(always)]
    fn poll_event(&mut self) -> Event  {
        todo!()
    }

    #[inline(always)]
    fn show(&mut self, parameters : WindowManagerShowParameters) {
        todo!()
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
    fn get_screen_list(&self) -> &ScreenList {
        &self.screens
    }

    #[inline(always)]
    fn set_title(&mut self, title:&str) {
        todo!()
    }

    #[inline(always)]
    fn set_size(&mut self, size : (u32, u32)) {
        todo!()
    }

    #[inline(always)]
    fn set_position(&mut self, position : (i32,i32)) {
        todo!()
    }

    #[inline(always)]
    fn set_pointer_position(&mut self, position : (i32,i32)) {
        todo!()
    }

    #[inline(always)]
    fn hide_pointer(&mut self) {
        todo!()
    }

    #[inline(always)]
    fn show_pointer(&mut self) {
        todo!()
    }

    #[inline(always)]
    fn confine_pointer(&mut self) {
        todo!()
    }

    #[inline(always)]
    fn release_pointer(&mut self) {
        todo!()
    }

    #[inline(always)]
    fn enable_autorepeat(&mut self) {
        todo!()
    }

    #[inline(always)]
    fn disable_autorepeat(&mut self) {
        todo!()
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

    

    

    

   
}