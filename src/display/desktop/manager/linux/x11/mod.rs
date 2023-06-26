use std::cell::RefCell;
use std::ffi::{CString, c_int, CStr, c_void};
use std::panic::catch_unwind;
use std::ptr::{null_mut};
use std::thread;

use crate::display::desktop::event::Event;
use crate::display::desktop::event::window::EventWindow;
use crate::display::desktop::manager::{WindowManager};
use crate::display::desktop::manager::linux::x11::cbind::xinput::{XNQueryInputStyle, XIMStyle, XIMPreeditNothing, XIMStatusNothing, XNInputStyle, XNClientWindow, XNFocusWindow};
use crate::display::desktop::property::{WindowProperty, WindowPositionOption, get_absolute_position_from_relative, PointerMode, KeyboardMode};
use crate::display::desktop::window::{Window, WindowShowOption};
use crate::error::StudioError;
use self::cbind::structs::XEvent;
use self::cbind::xinput::{XIM, XIC, XIMStyles };

/// Contains X11 C Bind
pub(crate) mod cbind;

/// Contains X11 screen fetch function
pub(crate) mod screen;

/// Contains X11 atoms
pub(crate) mod atom;

/// Contains X11 Events handling
pub(crate) mod event;

use cbind::{attributes::*, constants::*, functs::*, structs::* };
use cfg_boost::{target_cfg, match_cfg};


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
pub static mut X11_SUPPORTED : Option<bool> = Option::None;

/// Parent informations.
pub(super) struct X11WindowManagerParent {
    /// Display handle of parent.
    pub display : *mut X11Display,

    /// Window handle of parent.
    pub window : *mut X11Handle,
}

pub(crate) struct X11WindowManager {
    /// Used to fetch X11 events
    pub(crate) x_event : XEvent,  

    /// Xinput method
    xim : XIM,

    /// Xinput context
    xic : XIC,

    /// Retained events that will be sent next poll_event 
    pub(crate) retained_events : RefCell<Vec<Event>>,

    /// C-compatible string for window title
    wm_title : CString,

    /// Display connection pointer
    pub(crate) display : *mut X11Display,

    /// Window handle pointer. Is wrapped in option to make sure it is initialized when called.
    pub(crate) window : Option<*mut X11Handle>,

    /// Atoms for handling x11 window properties
    pub(crate) atoms : X11Atoms,

    /// Count of event to poll (immediate only)
    #[cfg(feature = "immediate")]
    pub(crate) event_count : usize,

    /// Event given as reference for immediate mode
    #[cfg(feature = "immediate")]
    event : Event,

    /// [Window] properties
    pub(super) property : WindowProperty,

    /// Window is fullscreen.
    pub(crate) fullscreen : bool,   

    /// Parent of the window if any (retained only)
    #[cfg(not(feature = "immediate"))]
    parent : Option<X11WindowManagerParent>,

    /// Indicate if window need to be recreated/refreshed.
    need_refresh:bool,

}

impl WindowManager for X11WindowManager {
    fn new() -> Result<Self, StudioError> {

        unsafe{
            let display = XOpenDisplay(std::ptr::null());      // Display connection
            let atoms = X11Atoms::new(display);                         // X11 Atoms

            match_cfg! {
                !immediate:ft => {
                    Ok(X11WindowManager {
                        x_event: XEvent{ _type:0 }, 
                        retained_events: RefCell::new(Vec::new()),
                        wm_title: CString::new("").unwrap(), 
                        display,
                        window: Option::None,
                        atoms,
                        property: WindowProperty::new(),
                        fullscreen: false,
                        xim: 0,
                        xic: 0,
                        need_refresh: true,
                        parent: Option::None,
                    })
                },
                _ => {
                    Ok(X11WindowManager {
                        x_event: XEvent{ _type:0 }, 
                        retained_events: RefCell::new(Vec::new()),
                        wm_title: CString::new("").unwrap(), 
                        display,
                        window: Option::None,
                        atoms,
                        event_count: 0,
                        property: WindowProperty::new(),
                        fullscreen: false,
                        event: Event::None,
                        xim: 0,
                        xic: 0,
                        need_refresh: true,
                    })
                }
            }

            
        }
        
    }

    #[inline(always)]
    fn get_window_provider(&self) -> WindowProvider {
        WindowProvider::X11
    }

    target_cfg! {
        !immediate:ft => {  // Retained mode
            #[inline(always)]
            fn show(&mut self, option : WindowShowOption, parent : Option<&Window>) {
                assert!(!self.property.created);    // Make sure window wasn't created before call.
        
                self.property.show_option = Some(option);
        
                self.parent = match parent{
                    Some(parent) => Some(X11WindowManagerParent { display: parent.manager.get_x11_wm().display, window: parent.manager.get_x11_wm().window.unwrap() }),
                    Option::None => Option::None,
                };
        
                self.create_window();
            }

            #[inline(always)]
            fn poll_event(&mut self) -> Event  {    // Always wait for events.
                self.sync();
                self.fetch_event()
            }
        },
        immediate:ft => {   // Immediate mode
            #[inline(always)]
            fn show(&mut self) {
                assert!(!self.property.created);    // Make sure window wasn't created before call.        
                self.create_window();
            }

            #[inline(always)]
            fn poll_event(&mut self) -> &Event  {   // Never wait for events
                if self.event == Event::None {
                    self.sync();
                    self.event_count = self.get_event_count();
                }
                self.event = self.fetch_event();
                &self.event
            }
        }
    }

    fn push_event(&self, retain: Event){
        self.retained_events.borrow_mut().push(retain);
    }

   

    #[inline(always)]
    fn restore(&mut self) {
        todo!()
    }

    #[inline(always)]
    fn close(&mut self) {
        unsafe {
            if let Some(window) = self.window {
                XDestroyWindow(self.display, window);
                self.property.created = false;
                self.window = Option::None;   // Delete window pointer.
            }

            // Send closed event to window
            self.push_event(Event::Window(EventWindow::Closed))
        }
    }

    #[inline(always)]
    fn refresh(&mut self) {
        
        if self.need_refresh && self.property.created {
            unsafe {
                if let Some(window) = self.window {
                    XDestroyWindow(self.display, window);
                    self.property.created = false;
                    self.window = Option::None;   // Delete window pointer.
                }

                self.create_window();
            }
        }

        self.need_refresh = false;

    }

    #[inline(always)]
    fn set_title(&mut self, title : &String){
        unsafe {
            self.property.title = title.to_string();
            self.wm_title = CString::from_vec_unchecked(title.as_bytes().to_vec());

            if let Some(window) = self.window {
                XStoreName(self.display, window, self.wm_title.as_ptr() as *mut i8);
            }
        }
    }

    #[inline(always)]
    fn set_position(&mut self, option : WindowPositionOption){
        unsafe {
            self.property.relative_position = option.clone();

            if let Some(window) = self.window {
                self.property.position = self.get_position_from_relative();
                XMoveWindow(self.display, window, self.property.position.0, self.property.position.1);
            }
        }
    }

    #[inline(always)]
    fn set_size(&mut self, size : &(u32,u32)){
        unsafe {
            if let Some(window) = self.window {
                // Keep real window position
                let position = X11WindowManager::get_x11_window_position(self.display, window);

            
                XResizeWindow(self.display, window, size.0, size.1);
            
                // Reposition window since resize put it back at 0,0
                XMoveWindow(self.display, window, position.0, position.1);
            }
        }
    }

    #[inline(always)]
    fn show_decoration(&mut self){
        self.property.decoration = true;
        self.need_refresh = true;
    }

    #[inline(always)]
    fn hide_decoration(&mut self){
        self.property.decoration = false;
        self.need_refresh = true;
    }

    #[inline(always)]
    fn minimize(&mut self){
        self.property.minimized = true;
        self.property.maximized = false;
        self.property.fullscreen = Option::None;
        self.need_refresh = true;
    }

    #[inline(always)]
    fn maximize(&mut self){
        self.property.minimized = false;
        self.property.maximized = true;
        self.property.fullscreen = Option::None;
    }

    #[inline(always)]
    fn set_keyboard_mode(&mut self, mode : KeyboardMode){
        self.property.keyboard.mode = mode;
    }

    #[inline(always)]
    fn set_keyboard_auto_repeat(&mut self, auto_repeat : bool){
        self.property.keyboard.auto_repeat = auto_repeat;
        self.set_xauto_repeat();
    }

    #[inline(always)]
    fn set_pointer_position(&mut self, position : (i32, i32)){
        unsafe {
            if let Some(window) = self.window {
                XWarpPointer(self.display, window, window, 0, 0, 
                    0, 0, position.0,  position.1);
            }
        }
    }

    #[inline(always)]
    fn show_pointer(&mut self){
        unsafe {
            if let Some(window) = self.window {
                XFixesShowCursor(self.display, window);
            }
            self.property.pointer.visible = true;     
        }
    }

    #[inline(always)]
    fn hide_pointer(&mut self){
        unsafe {
            self.property.pointer.visible = false;
            if let Some(window) = self.window {
                XFixesHideCursor(self.display, window);
            }
        }
    }

    #[inline(always)]
    fn confine_pointer(&mut self){
        unsafe {
            self.property.pointer.confined = true;
            if let Some(window) = self.window {
                XGrabPointer(self.display, window, true, 
        0, GrabModeAsync.try_into().unwrap(), GrabModeAsync.try_into().unwrap(), window, 0, CurrentTime);
            }
        }
    }

    #[inline(always)]
    fn release_pointer(&mut self){
        unsafe {
            self.property.pointer.confined = false;
            XUngrabPointer(self.display, CurrentTime);
        }
    }

    #[inline(always)]
    fn get_properties(&self) -> &WindowProperty {
        &self.property
    }
    
    #[inline(always)]
    fn set_fullscreen(&mut self, fsmode : crate::display::desktop::property::FullScreenMode){
        self.property.fullscreen = Some(fsmode);
        self.need_refresh = true;
    }

    #[inline(always)]
    fn set_pointer_mode(&mut self, mode : &crate::display::desktop::property::PointerMode){
        match mode {
            PointerMode::Acceleration => self.set_pointer_position(self.property.center),
            _ => {},
        };
        self.property.pointer.mode = *mode;
    }

    #[inline(always)]
    fn is_key_shift_down(state : u32) -> bool {
        state & ShiftMask as u32 > 0
    }

    #[inline(always)]
    fn is_key_ctrl_down(state : u32) -> bool {
        state & ControlMask as u32 > 0
    }

    #[inline(always)]
    fn is_key_alt_down(state : u32) -> bool {
        state & Mod1Mask as u32 > 0
    }

    #[inline(always)]
    fn is_key_meta_down(state : u32) -> bool {
        state & Mod3Mask as u32 > 0
    }

    #[inline(always)]
    fn is_key_command_down(state : u32) -> bool {
        state & Mod4Mask as u32 > 0
    }

    #[inline(always)]
    fn is_key_hyper_down(state : u32) -> bool {
        state & Mod5Mask as u32 > 0
    }

    #[inline(always)]
    fn is_capslock_on(state : u32) -> bool {
        state & LockMask as u32 > 0
    }

    #[inline(always)]
    fn is_numlock_on(state : u32) -> bool {
        state & Mod2Mask as u32 > 0
    }

    

    
}


impl X11WindowManager {

    /// Create the window according to window properties.
    #[inline(always)]
    fn create_window(&mut self){
        unsafe {
            // Get root window according to parent.
            let root = match_cfg! {
                !immediate:ft => {  // Retained mode
                    match &self.parent {
                        Some(parent) => parent.window,
                        Option::None => Self::get_x11_default_root_window(self.display),
                    };
                },
                _ => {  // Immediate mode
                    Self::get_x11_default_root_window(self.display)
                }
            };            

            let window = XCreateSimpleWindow(self.display, root, self.property.position.0,self.property.position.1,
                self.property.size.0, self.property.size.1, 0, 0, 0);

            // Set window Type to normal
            x11_change_property!(self.display, window, self.atoms, _NET_WM_WINDOW_TYPE, _NET_WM_WINDOW_TYPE_NORMAL);

            // Set window protocols to capture window closing
            XSetWMProtocols(self.display, window, &mut self.atoms.WM_DELETE_WINDOW, 1);

            // Allowed actions
            x11_change_property!(self.display, window, self.atoms, _NET_WM_ALLOWED_ACTIONS, _NET_WM_ACTION_FULLSCREEN, _NET_WM_ACTION_MINIMIZE, _NET_WM_ACTION_CHANGE_DESKTOP,
                _NET_WM_ACTION_CLOSE, _NET_WM_ACTION_ABOVE, _NET_WM_ACTION_BELOW);

            match &self.property.fullscreen{
                Some(_) => {
                    // TODO: Set fullscreen according to mode.
                    // Set as fullscreen
                    x11_change_property!(self.display, window, self.atoms, _NET_WM_STATE, _NET_WM_STATE_MAXIMIZED_HORZ, _NET_WM_STATE_MAXIMIZED_VERT, _NET_WM_STATE_FULLSCREEN);
                    self.fullscreen = true;
                },
                Option::None => self.fullscreen = false,     // No fullscreen mode
            }

            // Mask of events to receive
            XSelectInput(self.display, window, EVENT_MASK);

            // Flush buffer
            XFlush(self.display);

            // Create XIC and XIM
            self.create_xim_xic(self.display, window);

            // Set window in self
            self.window = Some(window);

            // Set window created flag to true.
            self.property.created = true;            

            // Send created event to window
            self.push_event(Event::Window(EventWindow::Created))
        }
    }

    /*
    /// Map the window according to window properties.
    #[inline(always)]
    fn map_window(&mut self, window : *mut X11Handle){
        match &self.property.parent{
            Some(parent) => {
                match parent.1 {
                    SubWindowOption::Normal => unsafe { XMapWindow(self.display, window) },
                    SubWindowOption::Top => unsafe { XMapRaised(self.display, window) },
                    SubWindowOption::Modal =>  {
                        unsafe { XMapRaised(self.display, window) }
                        // Send modal showed to parent
                       parent.0.manager.push_event(Event::Window(EventWindow::ModalShowed));
                    },
                }
            },
            Option::None => unsafe { XMapWindow(self.display, window) },
        }

        self.mapped = true;
    }
    */

    
    /// Set X11 keyboard autorepeat
    fn set_xauto_repeat(&self){
        unsafe {
            if self.property.keyboard.auto_repeat {
                XAutoRepeatOn(self.display);
            } else {
                XAutoRepeatOff(self.display);
            }
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

    /// Get the window position from the relative position.
    fn get_position_from_relative(&self) -> (i32, i32) {
        let parent_pos_size : ((i32,i32),(u32,u32)) = match_cfg! {
            !immediate:ft => {     // Retained mode
                match &self.parent {   // Get parent position and size if parent
                    Some(parent) => {
                        // Get parent window attributes
                        let parent_attr = Self::get_x11_window_attributes(parent.display, parent.window);
        
                        (Self::get_x11_window_position(parent.display, parent.window), (parent_attr.width as u32, parent_attr.height as u32))
                    },
                    Option::None => ((0,0),(0,0)),
                };        
            }, 
            _ => ((0,0),(0,0)), // Immediate mode
        };
        
        get_absolute_position_from_relative(self.property.size, &self.property.relative_position, parent_pos_size)

    }

    pub fn is_supported() -> bool { 

        unsafe {
            match X11_SUPPORTED {
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
                            X11_SUPPORTED = Some(value);
                            value
                        },
                        Err(_) => {
                            // Not supported
                            X11_SUPPORTED = Some(false);
                            false
                        },
                    }
                },
            }        
        }
    }

    /// Open XIM and XIC connection.
    /// 
    /// Reference(s)
    /// <https://handmade.network/forums/articles/t/2834-tutorial_a_tour_through_xlib_and_related_technologies>
    #[inline(always)]
    fn create_xim_xic(&mut self, display : *mut X11Display, window : *mut X11Handle) {

        unsafe {
            let x_input_method = XOpenIM(display, 0, 0 as *mut i8, 0 as *mut i8);
            assert!(x_input_method > 0, "Input Styles could not be retrieved!");

            let mut styles: *mut XIMStyles = null_mut();
            let input_style = CStr::from_bytes_with_nul(XNQueryInputStyle.as_bytes()).unwrap();

            assert!(XGetIMValues(x_input_method, input_style.as_ptr(), &mut styles, null_mut()) == null_mut() as *mut i8, "Input Styles could not be retrieved");

            let mut best_match_style : XIMStyle  = 0;
            for i in 0..(*styles).count_styles {
                let this_style : *mut XIMStyle = (*styles).supported_styles.offset(i as isize);
                if *this_style == (XIMPreeditNothing as u64 | XIMStatusNothing as u64)
                {
                    best_match_style = *this_style;
                    break;
                }
            }
                
            XFree(styles as *mut c_void);

            assert!(best_match_style > 0, "No matching input style could be determined");

            let input_style = CStr::from_bytes_with_nul(XNInputStyle.as_bytes()).unwrap();
            let client_window = CStr::from_bytes_with_nul(XNClientWindow.as_bytes()).unwrap();
            let focus_window = CStr::from_bytes_with_nul(XNFocusWindow.as_bytes()).unwrap();

            let x_input_context = XCreateIC(x_input_method, input_style.as_ptr(), best_match_style,
            client_window.as_ptr(), window, focus_window.as_ptr(), window, null_mut());

            assert!(x_input_context > 0, "Input Context could not be created");

            XSetICFocus(x_input_context);

            self.xim = x_input_method;
            self.xic = x_input_context;
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