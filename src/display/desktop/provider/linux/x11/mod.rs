use std::ffi::{CString, c_int};
use std::panic::catch_unwind;
use std::ptr::{null_mut};
use std::thread;

use crate::display::desktop::manager::WindowManager;
use crate::display::desktop::{event::Event, property::WindowProperty};
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

use cbind::{attributes::*, constants::*, functs::*, structs::* };


use super::{WindowProvider}; 
use atom::X11Atoms;


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

/// Static cache to know if X11 is supported
#[doc(hidden)]
pub static mut X11Supported : Option<bool> = Option::None;

pub(crate) struct X11WindowManager {
    /// Used to fetch X11 events
    pub(crate) x_event : XEvent,    

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

    /// Keyboard autorepeat
    pub(crate) auto_repeat : bool,

    /// Window position
    position : (i32, i32),

    /// Window size
    size : (u32, u32),

    /// Pointer is confined
    pub(crate) pointer_confined : bool,

    /// Pointer is visible
    pub(crate) pointer_visible : bool,

    /// Window is fullscreen
    pub(crate) fullscreen : bool,

    /// Window is maximized
    pub(crate) maximized : bool,

    /// Window is minimized
    pub(crate) minimized : bool,

    /// Window has been created.
    pub(crate) created : bool,

    /// Window has been mapped.
    pub(crate) mapped : bool,

}

impl WindowManager for X11WindowManager {
    fn new() -> Result<Self, StudioError> {

        unsafe{
            let display = XOpenDisplay(std::ptr::null());      // Display connection
            let atoms = X11Atoms::new(display);                         // X11 Atoms
            
            Ok(X11WindowManager {
                x_event: XEvent{ _type:0 }, 
                retained_events: Vec::new(),
                wm_title: CString::new("").unwrap(), 
                display,
                window: null_mut(),
                atoms,
                event_count: 0,
                auto_repeat: false,
                pointer_confined: false,
                pointer_visible: true,
                position: (0,0),
                size: (640,480),
                fullscreen: false,
                maximized: false,
                minimized: false,
                created : false,
                mapped: false,
            })
        }
        
    }

    #[inline(always)]
    fn get_window_provider(&self) -> WindowProvider {
        WindowProvider::X11
    }

    #[inline(always)]
    fn poll_event(&mut self) -> Event  {

        if self.retained_events.len() > 0 { // Pop event from retained
            self.retained_events.pop().unwrap() 
        } else {
            // Get count to poll
            if self.event_count == 0 {
                self.sync();
                self.event_count = self.get_event_count();
            }
            self.get_event()
        }
        
    }

    fn push_event(&mut self, retain: Event){
        self.retained_events.push(retain);
    }

    #[inline(always)]
    fn show(&mut self, property : &WindowProperty) {
        if !self.created {  // Create window if not created
            self.create_window(property);
        } 
        
        if !self.mapped{
            self.map_window(property);
        }
    }

    #[inline(always)]
    fn close(&mut self) {
        unsafe {
            XDestroyWindow(self.display, self.window);
            self.created = false;
            self.mapped = false;
            self.window = null_mut();   // Delete window pointer.
        }
    }

    #[inline(always)]
    fn hide(&mut self) {
        unsafe {
            XUnmapWindow(self.display, self.window);
            self.mapped = false;
        }
    }

    #[inline(always)]
    fn set_title(&mut self, title : &String) -> bool {
        unsafe {
            self.wm_title = CString::from_vec_unchecked(title.as_bytes().to_vec());
            XStoreName(self.display, self.window, self.wm_title.as_ptr() as *mut i8);
        }
        false
    }

    #[inline(always)]
    fn set_position(&mut self, position : (i32,i32)) -> bool {
        unsafe {
            XMoveWindow(self.display, self.window, position.0, position.1);
        }
        false
    }

    #[inline(always)]
    fn set_size(&mut self, size : &(u32,u32)) -> bool {
        unsafe {
            // Keep real window position
            let position = X11WindowManager::get_x11_window_position(self.display, self.window);

            XResizeWindow(self.display, self.window, size.0, size.1);
            
            // Reposition window since resize put it back at 0,0
            self.set_position(position);

        }
        false
    }

    #[inline(always)]
    fn show_decoration(&mut self) -> bool {
        true
    }

    #[inline(always)]
    fn hide_decoration(&mut self) -> bool {
        true
    }

    #[inline(always)]
    fn minimize(&mut self) -> bool {
        true
    }

    #[inline(always)]
    fn maximize(&mut self) -> bool {
        false
    }

    #[inline(always)]
    fn enable_autorepeat(&mut self) -> bool {
        self.auto_repeat = true;
        false
    }

    #[inline(always)]
    fn disable_autorepeat(&mut self) -> bool {
        self.auto_repeat = false;
        false
    }

    #[inline(always)]
    fn set_pointer_position(&mut self, position : &(i32, i32)) -> bool {
        unsafe {
            XWarpPointer(self.display, self.window, self.window, 0, 0, 
                0, 0, position.0,  position.1);
        }
        false
    }

    #[inline(always)]
    fn show_pointer(&mut self) -> bool {
        unsafe {
            if !self.pointer_visible {    // Make sure X hide cursor was called prior to show.
                XFixesShowCursor(self.display, self.window);
                self.pointer_visible = true;
            }       
        }
        false
    }

    #[inline(always)]
    fn hide_pointer(&mut self) -> bool {
        unsafe {
            if self.pointer_visible {
                self.pointer_visible = false;
                XFixesHideCursor(self.display, self.window);
            }
        }
        false
    }

    #[inline(always)]
    fn confine_pointer(&mut self) -> bool {
        unsafe {
            self.pointer_confined = true;
            XGrabPointer(self.display, self.window, true, 
            0, GrabModeAsync.try_into().unwrap(), GrabModeAsync.try_into().unwrap(), self.window, 0, CurrentTime);
        }
        false
    }

    #[inline(always)]
    fn release_pointer(&mut self) -> bool {
        unsafe {
            self.pointer_confined = false;
            XUngrabPointer(self.display, CurrentTime);
        }
        false
    }

    #[inline(always)]
    fn get_window_handle(&self) -> Option<*const usize> {
        if self.window == null_mut() {
            Option::None
        }else {
            Some(self.window as *const usize)
        }
    }  

    #[inline(always)]
    fn get_display_handle(&self) -> Option<*const usize> {
        if self.display == null_mut() {
            Option::None
        }else {
            Some(self.display as *const usize)
        }
    }

    
}


impl X11WindowManager {

    /// Create the window according to window properties.
    #[inline(always)]
    fn create_window(&mut self, property : &WindowProperty){
        unsafe {
            // Get root window according to parent.
            let root = match &property.parent{
                Some(parent) => parent.borrow().get_window_handle().unwrap() as *mut u64,
                Option::None => Self::get_x11_default_root_window(self.display),
            };

            self.window = XCreateSimpleWindow(self.display, root, property.position.0,property.position.1,
                property.size.0, property.size.1, 0, 0, 0);

            // Set window Type to normal
            x11_change_property!(self.display, self.window, self.atoms, _NET_WM_WINDOW_TYPE, _NET_WM_WINDOW_TYPE_NORMAL);

            // Set window protocols to capture window closing
            XSetWMProtocols(self.display, self.window, &mut self.atoms.WM_DELETE_WINDOW, 1);

            // Allowed actions
            x11_change_property!(self.display, self.window, self.atoms, _NET_WM_ALLOWED_ACTIONS, _NET_WM_ACTION_FULLSCREEN, _NET_WM_ACTION_MINIMIZE, _NET_WM_ACTION_CHANGE_DESKTOP,
                _NET_WM_ACTION_CLOSE, _NET_WM_ACTION_ABOVE, _NET_WM_ACTION_BELOW);

            match &property.fullscreen{
                Some(_) => {
                    // TODO: Set fullscreen according to mode.
                    // Set as fullscreen
                    x11_change_property!(self.display, self.window, self.atoms, _NET_WM_STATE, _NET_WM_STATE_MAXIMIZED_HORZ, _NET_WM_STATE_MAXIMIZED_VERT, _NET_WM_STATE_FULLSCREEN);
                    self.fullscreen = true;
                },
                Option::None => self.fullscreen = false,     // No fullscreen mode
            }

            // Mask of events to receive
            XSelectInput(self.display, self.window, EVENT_MASK);

            // Flush buffer
            XFlush(self.display);

            // Set window created flag to true.
            self.created = true;
        }
    }

    /// Map the window according to window properties.
    #[inline(always)]
    fn map_window(&mut self, property : &WindowProperty){
        match &property.parent{
            Some(parent) => {
                match property.subwindow_option {
                    Some(option) => match option{
                        crate::display::desktop::property::SubWindowOption::Normal =>  unsafe { XMapWindow(self.display, self.window) },
                        crate::display::desktop::property::SubWindowOption::Top =>  unsafe { XMapRaised(self.display, self.window) },
                        crate::display::desktop::property::SubWindowOption::Modal => {
                            unsafe { XMapRaised(self.display, self.window) }
                            // Lock parent
                            parent.borrow_mut().property.locked = true;
                        },
                    },
                    Option::None => unsafe { XMapWindow(self.display, self.window) },
                }
            },
            Option::None => unsafe { XMapWindow(self.display, self.window) },
        }

        self.mapped = true;
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

    pub fn is_supported() -> bool { 

        unsafe {
            match X11Supported {
                Some(support) => support,
                Option::None => {
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
                        Ok(value) => {
                            X11Supported = Some(value);
                            value
                        },
                        Err(_) => {
                            // Not supported
                            X11Supported = Some(false);
                            false
                        },
                    }
                },
            }        
        }
    }
}

impl Drop for X11WindowManager {
    fn drop(&mut self) {
        unsafe {
            // Close display server connection.
            XCloseDisplay(self.display);
        }
    }
}
/*
// Contains X11 Window manager
//pub mod manager;

/// Contains X11 Window implementation
//pub mod window;
//pub use window::X11Window as X11Window;

/// Contains X11 Window manager events bind
//pub(crate) mod event;

/// X11 Window implementation.
pub(crate) struct X11Window<'window> {
    /// Used to fetch X11 events
    pub(crate) x_event : XEvent,    

    /// X11 window properties
    pub(crate) property : WindowProperty<'window>,

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
}

impl<'window> Window for X11Window<'window> {
    #[inline(always)]
    fn new() -> Result<Self, StudioError>  {   // This function is not supported for X11Window
        todo!()
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
    fn set_property(&mut self, property : WindowPropertySet) -> Result<usize, StudioError> {
        self.set_window_properties(&[property])
    }

    #[inline(always)]
    fn set_properties(&mut self, properties : &[WindowPropertySet]) -> Result<usize, StudioError> {
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

impl<'window> X11Window<'window> {
    fn from(linux_window : &LinuxWindow) -> Result<Self, StudioError>  {   // This function is not supported for X11Window
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
                position: WindowPositionOption::Desktop((0,0)),
                linux_window,
            })
        }
    }

    /// 
    pub(crate) fn create_window(&mut self){

    }
    

    /// Recreate the window.
    pub(crate) fn recreate_window(&mut self){

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
impl<'window> Drop for X11Window<'window> {
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
*/