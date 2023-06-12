use std::cell::RefCell;
use std::ffi::{CString, c_int};
use std::panic::catch_unwind;
use std::ptr::null_mut;
use std::rc::Rc;
use std::thread;

use crate::display::desktop::property::{WindowPropertySet, WindowPositionOption};
use crate::display::desktop::window::WindowType;
use crate::display::desktop::{window::Window, event::Event, property::WindowProperty};
use crate::error::StudioError;
use self::cbind::structs::XEvent;

/// Contains X11 C Bind
pub(crate) mod cbind;

/// Contains X11 screen fetch function
pub(crate) mod screen;

/// Contains X11 atoms
pub(crate) mod atom;

/// Contains X11 Events handling
pub(crate) mod event;

/// Contains X11 Property set handling
pub(crate) mod property;

use cbind::{attributes::*, constants::*, functs::*, structs::* };

use super::super::super::screen::{ScreenList, Screen};
use super::WindowProvider; 
use atom::X11Atoms;
use screen::get_x11_screen_list;

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


// Contains X11 Window manager
//pub mod manager;

/// Contains X11 Window implementation
//pub mod window;
//pub use window::X11Window as X11Window;

/// Contains X11 Window manager events bind
//pub(crate) mod event;

/// X11 Window implementation.
pub(crate) struct X11Window {
    /// Used to fetch X11 events
    pub(crate) x_event : XEvent,    

    /// X11 window properties
    pub(crate) property : WindowProperty,

    /// X11 window relative position used for window creation
    pub(crate) position: WindowPositionOption,

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

    /// RC RefCell reference to LinuxWindow
    pub(crate) rc_ref : Option<Rc<RefCell<WindowType>>>,
}

impl Window for X11Window {
    #[inline(always)]
    fn new() -> Result<Self, StudioError>  {   // This function is not supported for X11Window
        unsafe{
            let display = XOpenDisplay(std::ptr::null());     // Display connection
            let mut atoms = X11Atoms::new(display);     // X11 Atoms
            
            Ok(X11Window {
                x_event: XEvent{ _type:0 }, 
                retained_events: Vec::new(),
                wm_title: CString::new("").unwrap(), 
                display,
                window: null_mut(),
                atoms,
                event_count: 0,
                property: WindowProperty::new(),
                rc_ref: Option::None,
                position: WindowPositionOption::Desktop((0,0)),
            })
        }
    }

    #[inline(always)]
    fn show(&mut self) -> Result<bool, StudioError> {
        todo!()
    }

    #[inline(always)]
    fn hide(&mut self) {
        todo!()
    }

    #[inline(always)]
    fn close(&mut self) {
        todo!()
    }

    #[inline(always)]
    fn poll_event(&mut self) -> Event {
        // Get count to poll
        if self.event_count == 0 {
            self.sync();
            self.event_count = self.get_event_count();
        }
        self.get_event()
    }

    #[inline(always)]
    fn get_provider(&self) -> WindowProvider {
        WindowProvider::X11
    }

    #[inline(always)]
    fn get_properties(&self) -> &WindowProperty {
        &self.property
    }

    #[inline(always)]
    fn set_property(&mut self, property : WindowPropertySet) -> Result<usize, (WindowPropertySet, StudioError)> {
        self.set_window_properties(&[property])
    }

    #[inline(always)]
    fn set_properties(&mut self, properties : &[WindowPropertySet]) -> Result<usize, (WindowPropertySet, StudioError)> {
        self.set_window_properties(properties)
    }

    #[inline(always)]
    fn get_window_handle(&self) -> Option<*const usize> {
        if self.window != null_mut() {
            Some(self.window as *const usize)
        } else {
            Option::None
        } 
    }

    #[inline(always)]
    fn get_display_handle(&self) -> Option<*const usize> {
        if self.display != null_mut() {
            Some(self.display as *const usize)
        } else {
            Option::None
        } 
    }

    

    

}

impl X11Window {

    /// 
    pub(crate) fn create_window(&mut self){

    }
    

    /// Recreate the window.
    pub(crate) fn recreate_window(&mut self){

    }

    pub(crate) fn get_window_rcref(&self) -> Rc<RefCell<WindowType>> {
        self.rc_ref.unwrap()
    }

    /// Get self properties as mutable
    pub(crate) fn get_properties_mut(&mut self) -> &mut WindowProperty{
        &mut self.property
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

/// [Drop] trait implementation for [X11WindowManager].
impl Drop for X11Window {
    fn drop(&mut self) {
        unsafe {
            // Close display server connection.
            XCloseDisplay(self.display);
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