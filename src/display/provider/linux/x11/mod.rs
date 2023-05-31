use std::thread;
use std::ffi::{CStr, CString};
use std::os::raw::{ c_int, c_long, c_uint, c_ulong, c_char, c_uchar, c_short, c_void };
use std::ptr::null_mut;
use std::{panic::catch_unwind};


use crate::display::desktop::cursor::CursorMode;
use crate::display::desktop::window::{WindowProperty, Window, FullscreenMode};

use bind::{XFree, XGetAtomName, XFlush, XScreenOfDisplay};

use super::super::super::screen::{ScreenList, Screen};
use crate::display::{ provider::WindowProvider};
use crate::display::event::WindowEventWindow;
use crate::display::provider::linux::x11::constant::{GrabModeAsync};
use crate::display::{provider::linux::x11::{bind::{XDefaultRootWindow, XCreateSimpleWindow, XMapWindow, XSelectInput, XSync, XEventsQueued}, 
    constant::{KeyPressMask, ButtonPressMask, ExposureMask, KeyPress, KeyRelease, ButtonPress, MotionNotify, LeaveNotify, 
    ButtonRelease, EnterNotify, FocusIn, FocusOut, KeymapNotify, Expose, GraphicsExpose, NoExpose, VisibilityNotify, 
    CreateNotify, DestroyNotify, UnmapNotify, MapNotify, MapRequest, ReparentNotify, ConfigureNotify, ConfigureRequest, 
    GravityNotify, CirculateNotify, CirculateRequest, PropertyNotify, SelectionClear, SelectionRequest, SelectionNotify, 
    ColormapNotify, ClientMessage, MappingNotify, GenericEvent}}, event::WindowEvent, event::WindowEventMouse, event::WindowEventKeyboard};

use self::atom::X11Atoms;
use self::attributes::{XWindowAttributes};
use self::bind::{XWarpPointer, XFixesHideCursor, XGrabPointer, XFixesShowCursor, XUngrabPointer, XGetWindowProperty, XStoreName, 
    XChangeProperty, XGetWindowAttributes, XTranslateCoordinates, 
    XResizeWindow, XMoveWindow, XDestroyWindow};
use self::constant::{CurrentTime, VisibilityUnobscured, PropModeReplace};
use self::event::{Atom, XEvent, X11Display, X11Handle};
use self::{ bind::{XOpenDisplay, XCloseDisplay, XNextEvent}, constant::{KeyReleaseMask, ButtonReleaseMask, LeaveWindowMask, EnterWindowMask, Button1MotionMask, PointerMotionMask, Button3MotionMask, Button2MotionMask, Button5MotionMask, Button4MotionMask, ButtonMotionMask, StructureNotifyMask, ResizeRedirectMask, VisibilityChangeMask, FocusChangeMask, PropertyChangeMask}};

/// Contains X11 contants definition
#[allow(unused)]                    // Remove unused variable notification
#[allow(non_upper_case_globals)]    // Imported C global aren't formatted according to convention.
pub mod constant;

/// Contains X11 Event definition
#[allow(unused)]                    // Remove unused variable notification
#[allow(non_snake_case)]            // Imported C members aren't formatted according to convention.
pub mod event;

/// Contains X11 Window attributes
pub mod attributes;

/// Contains X11 C functions Bind
pub mod bind;

/// Contains X11 screen fetch function
pub mod screen;

/// Contains X11 atoms
pub mod atom;

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

/// [X Window System](https://en.wikipedia.org/wiki/X_Window_System) implementation of [DisplayManager].
pub struct X11Window {

    /// Used to fetch X11 events
    x_event : XEvent,    

    /// C-compatible string for window title
    wm_title : CString,

    /// Display connection pointer
    display : *mut X11Display,

    /// Window handle pointer
    window : *mut X11Handle,

    /// Atoms for handling x11 window properties
    atoms : X11Atoms,

    /// Flag used to make sure XHideCursor was called prior to XShowCursor to prevent crash
    x_hide_cursor_flag : bool,

    /// Position and size for restoring window.
    restoration_position_size : ((i32,i32),(u32,u32)),

    /// Window properties.
    property : WindowProperty,

    /// Screen list
    screens : ScreenList,

    /// Count of event to poll
    event_count : usize,
}

/// Public members of [DisplayManagerX11].
impl X11Window {
    /// Create a new instance of DisplayManagerX11.
    pub(crate) fn new(width:u32, height:u32) -> X11Window {
        unsafe {
            let screens = ScreenList::from_provider(WindowProvider::X11).unwrap();
            let display = XOpenDisplay(std::ptr::null());     // Display connection
            let atoms = X11Atoms::new(display);     // X11 Atoms
            let position = Self::start_position(screens.get_primary_screen().unwrap(), (width, height));    // Window startup position
            let root = Self::get_x11_default_root_window(display);  // Root window

            X11Window{ 
                x_event: XEvent{ _type:0 }, 
                wm_title: CString::new("").unwrap(), 
                display, //: XOpenDisplay(std::ptr::null()), 
                window: Self::create_x11_window(display, root, &atoms, position, (width, height), false), 
                atoms,
                x_hide_cursor_flag: false,
                restoration_position_size: (position, (width, height)),
                property : WindowProperty::new(position, (width, height)),
                screens,
                event_count: 0, 
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

/// [Drop] trait implementation for [DisplayManagerX11].
impl Drop for X11Window {
    fn drop(&mut self) {
        unsafe {
            // Close display server connection.
            XCloseDisplay(self.display);
        }
    }
}

/// [DisplayManager] trait implementation for [DisplayManagerX11].
impl Window for X11Window {
    fn get_window_provider(&self) -> WindowProvider {
        WindowProvider::X11
    }

    fn get_window_property(&self) -> &WindowProperty {
        &self.property
    }

    fn set_cursor_mode(&mut self, mode : CursorMode)  {
         self.property.cursor.mode = mode;

         match mode {
            // Set cursor to center if Acceleration
            CursorMode::Acceleration => self.set_cursor_position(self.property.center),
            _ => todo!(),
        }
    }

    fn get_event_count(&self) -> usize {
        unsafe {
            XEventsQueued(self.display, 0).try_into().unwrap()
        }   
    }

    fn set_cursor_position(&mut self, position : (i32, i32)) {
        unsafe {
            XWarpPointer(self.display, self.window, self.window, 0, 0, 
                0, 0, position.0,  position.1);
        }
    }

    fn hide_cursor(&mut self) {
        unsafe {
            XFixesHideCursor(self.display, self.window);
            self.x_hide_cursor_flag = true;
        }
    }

    fn show_cursor(&mut self)  {
        unsafe {
            if self.x_hide_cursor_flag {    // Make sure X hide cursor was called prior to show.
                XFixesShowCursor(self.display, self.window);
                self.x_hide_cursor_flag = false;
            }       
        }
    }

    fn confine_cursor(&mut self) {
        unsafe {
            XGrabPointer(self.display, self.window, true, 
            0, GrabModeAsync.try_into().unwrap(), GrabModeAsync.try_into().unwrap(), self.window, 0, CurrentTime);
        }
    }

    fn release_cursor(&mut self) {
        unsafe {
            XUngrabPointer(self.display, CurrentTime);
        }
    }

    fn restore(&mut self) {
        unsafe {
            let states = self.get_x11_window_states_event();

            // Destroy current window
            XDestroyWindow(self.display, self.window);

            // Recreate window as normal
            self.window = X11Window::create_x11_window(self.display, XDefaultRootWindow(self.display), &self.atoms, self.restoration_position_size.0,
                self.restoration_position_size.1, false);   

            self.set_position(self.restoration_position_size.0);
        }        
    }

    fn set_title(&mut self, title : &str) {
        unsafe {
            self.wm_title = CString::from_vec_unchecked(title.as_bytes().to_vec());
            XStoreName(self.display, self.window, self.wm_title.as_ptr() as *mut i8);
        }
    }

    fn set_size(&mut self, size : (u32, u32)) {
        unsafe {
            // Keep real window position
            self.restoration_position_size.0 = X11Window::get_x11_window_position(self.display, self.window);

            XResizeWindow(self.display, self.window, size.0, size.1);
            
            // Reposition window since resize put it back at 0,0
            self.set_position(self.restoration_position_size.0);
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
                     &self.atoms, (0,0),   self.screens.get_primary_screen().unwrap().get_current_resolution(), true);      
                },
                FullscreenMode::PrimaryScreen => {

                },
                FullscreenMode::DesktopScreen => {

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
    fn poll_event(&mut self) -> WindowEvent {
        unsafe {
            // Get count to poll
            if self.event_count == 0 {
                self.sync();
                self.event_count = self.get_event_count();
            }

           

            // Only if we have something to poll
            if self.event_count > 0 {
                self.event_count -= 1;
                println!("EVC={}", self.event_count);
                
                XNextEvent(self.display, &mut self.x_event);
                let xevent = self.x_event; 
                
                match xevent._type {

                    // Keyboard key pressed
                    KeyPress => WindowEvent::Keyboard(WindowEventKeyboard::KeyDown(xevent._xkey._keycode)),

                    // Keyboard key release
                    KeyRelease=> WindowEvent::Keyboard(WindowEventKeyboard::KeyUp(xevent._xkey._keycode)),


                    ButtonPress=> { println!("Display({:p}), ButtonPress({})", self, xevent._type); WindowEvent::Unknown },
                    ButtonRelease=> { println!("Display({:p}), ButtonRelease({})", self, xevent._type); WindowEvent::Unknown },

                    // Cursor moved
                    MotionNotify=> {    
                        match self.property.cursor.mode {   
                            CursorMode::Pointer => WindowEvent::Mouse(WindowEventMouse::Moved((xevent._xmotion._x, xevent._xmotion._y))),
                            CursorMode::Acceleration => {
                                let position = (xevent._xmotion._x - self.property.center.0, 
                                    xevent._xmotion._y - self.property.center.1);
                                // Report acceleration only if movement occurred
                                if position.0 != 0 || position.1 != 0 {
                                    WindowEvent::Mouse(WindowEventMouse::Moved(position))
                                } else {
                                    WindowEvent::None
                                }
                            }
                        }
                    },

                    // Cursor entered window
                    EnterNotify=> WindowEvent::Window(WindowEventWindow::CursorEnter()),

                    // Cursor left window
                    LeaveNotify=> WindowEvent::Window(WindowEventWindow::CursorLeave()),

                    // Window got focus
                    FocusIn=> WindowEvent::Window(WindowEventWindow::Focus()),

                    // Window lost focus
                    FocusOut=> WindowEvent::Window(WindowEventWindow::Blur()),

                    KeymapNotify=> { println!("Display({:p}), KeymapNotify({})", self, xevent._type); WindowEvent::Unknown },

                    // Part of window need to be redrawed 
                    Expose=> { 
                        WindowEvent::Window(WindowEventWindow::Exposed((xevent._xexpose._x, xevent._xexpose._y), (xevent._xexpose._width as u32, xevent._xexpose._height as u32)))
                    },
                    GraphicsExpose=> { println!("Display({:p}), GraphicsExpose({})", self, xevent._type); WindowEvent::Unknown },
                    NoExpose=> { println!("Display({:p}), NoExpose({})", self, xevent._type); WindowEvent::Unknown },
                    VisibilityNotify=> { 
                        if xevent._xvisibility._state == VisibilityUnobscured {
                            WindowEvent::Window(WindowEventWindow::Shown())
                        } else {
                            WindowEvent::Window(WindowEventWindow::Hidden())
                        }
                    },
                    CreateNotify=> { println!("Display({:p}), CreateNotify({})", self, xevent._type); WindowEvent::Unknown },
                    DestroyNotify=> { println!("Display({:p}), DestroyNotify({})", self, xevent._type); WindowEvent::Unknown },
                    UnmapNotify=> { println!("Display({:p}), UnmapNotify({})", self, xevent._type); WindowEvent::Unknown },
                    MapNotify=> { println!("Display({:p}), MapNotify({})", self, xevent._type); WindowEvent::Unknown },
                    MapRequest=> { println!("Display({:p}), MapRequest({})", self, xevent._type); WindowEvent::Unknown },
                    ReparentNotify=> { println!("Display({:p}), ReparentNotify({})", self, xevent._type); WindowEvent::Unknown },

                    // Window position and/or size changed
                    ConfigureNotify=> { self.get_window_configuration_event() },

                    ConfigureRequest=> { println!("Display({:p}), ConfigureRequest({})", self, xevent._type); WindowEvent::Unknown },
                    GravityNotify=> { println!("Display({:p}), GravityNotify({})", self, xevent._type); WindowEvent::Unknown },

                    CirculateNotify=> { println!("Display({:p}), CirculateNotify({})", self, xevent._type); WindowEvent::Unknown },
                    CirculateRequest=> { println!("Display({:p}), CirculateRequest({})", self, xevent._type); WindowEvent::Unknown },
                    PropertyNotify=> { self.get_x11_window_states_event() },
                        
                    SelectionClear=> { println!("Display({:p}), SelectionClear({})", self, xevent._type); WindowEvent::Unknown },
                    SelectionRequest=> { println!("Display({:p}), SelectionRequest({})", self, xevent._type); WindowEvent::Unknown },
                    SelectionNotify=> { println!("Display({:p}), SelectionNotify({})", self, xevent._type); WindowEvent::Unknown },
                    ColormapNotify=> { println!("Display({:p}), ColormapNotify({})", self, xevent._type); WindowEvent::Unknown },
                    ClientMessage=> { println!("Display({:p}), ClientMessage({})", self, xevent._type); WindowEvent::Unknown },
                    MappingNotify=> { println!("Display({:p}), MappingNotify({})", self, xevent._type); WindowEvent::Unknown },
                    GenericEvent=> { println!("Display({:p}), GenericEvent({})", self, xevent._type); WindowEvent::Unknown },
                    _ => { println!("Display({:p}), _({})", self, xevent._type); WindowEvent::Unknown },
                }
            } else {
                WindowEvent::None   // Return None event
            }
        }
    }

    

    
}


/// Private members of [DisplayManagerX11].
impl X11Window {
    /// Create x11 Window according to position, size and if fullscreen or not.
    #[inline(always)]
    pub fn create_x11_window(display : *mut X11Display, root : *mut X11Handle, atoms : &X11Atoms, position : (i32, i32), 
        size : (u32,u32), fullscreen : bool) -> *mut X11Handle {
        unsafe {
            let window = XCreateSimpleWindow(display, root, position.0,position.1,
                    size.0, size.1, 4, 0, 0);

            // Set window Type to normal
            x11_change_property!(display, window, atoms, _NET_WM_WINDOW_TYPE, _NET_WM_WINDOW_TYPE_NORMAL);

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

    /// Get and set event from the X11 window configuration state.
    fn get_window_configuration_event(&mut self) -> WindowEvent {
        unsafe {
            let position = (self.x_event._xconfigure._x, self.x_event._xconfigure._y);
            let size = (self.x_event._xconfigure._width as u32, self.x_event._xconfigure._height as u32);
        
            // By default, set event as none.
            let mut event = WindowEvent::None;

            if position != self.property.position && size != self.property.size {
                event = WindowEvent::Window(WindowEventWindow::MovedResized(position, size));
            } else if position != self.property.position {
                event = WindowEvent::Window(WindowEventWindow::Moved(position));
            } else if size != self.property.size  {
                event = WindowEvent::Window(WindowEventWindow::Resized(size));
            }

            // Update window properties
            self.property.position = position;
            self.property.size = size;

            event
        }
    }

    /// Get and event from the X11 window state.
    /// 
    /// This function query XGetWindowProperty() to get Atoms used to identify min, max and fullscreen properties.
    #[inline(always)]
    fn get_x11_window_states_event(&mut self) -> WindowEvent {
        unsafe {
            // State values returned
            let mut hidden = false;
            let mut maximized = false;
            let mut fullscreen = false;

            // Used to capture XGetWindowProperty
            let mut actual_type_return : Atom = 0;
            let mut actual_format_return : c_int = 0; 
            let mut nitems_return : c_ulong = 0; 
            let mut bytes_after_return : c_ulong = 0; 
            let mut prop_return : *mut c_char = null_mut();

            XGetWindowProperty(self.display, self.window, self.atoms._NET_WM_STATE, 
                0, 1024, false, self.atoms.xa_atom, &mut actual_type_return, &mut actual_format_return, 
                &mut nitems_return, &mut bytes_after_return, &mut prop_return);
            
            // Only query if count of items > 0
            if nitems_return > 0 {
                // Converting according to actual_format_return
                match actual_format_return {
                    // 8 bits
                    8 => {
                        // Convert properties to u8
                        let states: &mut [u8] = core::slice::from_raw_parts_mut(prop_return as *mut u8, nitems_return as usize);
                        for state in states{
                            match *state as Atom {
                                state if self.atoms._NET_WM_STATE_HIDDEN == state => {
                                    hidden = true;
                                },
                                state if self.atoms._NET_WM_STATE_FULLSCREEN == state => {
                                    fullscreen = true;
                                },
                                state if self.atoms._NET_WM_STATE_MAXIMIZED_HORZ == state => {
                                    maximized = true;
                                },
                                state if self.atoms._NET_WM_STATE_MAXIMIZED_VERT == state => {
                                    maximized = true;
                                },
                                0 => {},   // Do nothing with 0 atoms
                                _ => {},
                            }
                        }
                        
                    },
                    // 16 bits
                    16 => {
                        // Convert properties to u16
                        let states: &mut [u16] = core::slice::from_raw_parts_mut(prop_return as *mut u16, nitems_return as usize);
                        for state in states{
                            match *state as Atom {
                                state if self.atoms._NET_WM_STATE_HIDDEN == state => {
                                    hidden = true;
                                },
                                state if self.atoms._NET_WM_STATE_FULLSCREEN == state => {
                                    fullscreen = true;
                                },
                                state if self.atoms._NET_WM_STATE_MAXIMIZED_HORZ == state => {
                                    maximized = true;
                                },
                                state if self.atoms._NET_WM_STATE_MAXIMIZED_VERT == state => {
                                    maximized = true;
                                },
                                0 => {},   // Do nothing with 0 atoms
                                _ => {},
                            }
                        }
                    },

                    // 32 bits
                    32 => {
                        // Convert properties to Atom
                        let states: &mut [Atom] = core::slice::from_raw_parts_mut(prop_return as *mut Atom, nitems_return as usize);
                        println!("States={:?}", states);
                        
                        for state in states{
                            match *state as Atom {
                                state if self.atoms._NET_WM_STATE_HIDDEN == state => {
                                    hidden = true;
                                },
                                state if self.atoms._NET_WM_STATE_FULLSCREEN == state => {
                                    fullscreen = true;
                                },
                                state if self.atoms._NET_WM_STATE_MAXIMIZED_HORZ == state => {
                                    maximized = true;
                                },
                                state if self.atoms._NET_WM_STATE_MAXIMIZED_VERT == state => {
                                    maximized = true;
                                },
                                0 => {},   // Do nothing with 0 atoms
                                // Print unknown state name
                                state => { 
                                    println!("State={:?}", CStr::from_ptr(XGetAtomName(self.display, state)).to_str().unwrap());
                                }
                            }
                        }
                    },

                    // Anything else is an error
                    _ => panic!("Wrong `actual_format_return` format size!"),
                }
            }

            // Free data returned.
            XFree(prop_return);

            // Event to return
            let mut event = WindowEvent::None;

            // Return event. By priority > Fullscreen > Minimized > Maximized > Restored > None
            if fullscreen {   // Send fullscreen if not already registered.
                if !self.property.is_fullscreen {
                    event = WindowEvent::Window(WindowEventWindow::Fullscreen());
                }
            } else if hidden {   // Send minimized if not already registered.
                    if !self.property.is_minimized {
                        event = WindowEvent::Window(WindowEventWindow::Minimized());
                    }
            } else if maximized {   // Send maximized if not already registered.
                if !self.property.is_maximized {
                    event = WindowEvent::Window(WindowEventWindow::Maximized());
                }
            } else {    // Send restore if not already registered.
                if self.property.is_fullscreen != fullscreen || 
                    self.property.is_maximized != maximized || 
                    self.property.is_minimized != hidden {
                        event = WindowEvent::Window(WindowEventWindow::Restored());
                    }
            }

            // Update window properties
            self.property.is_fullscreen = fullscreen;
            self.property.is_maximized = maximized;
            self.property.is_minimized = hidden;

            event
        }
    }
    
}