use std::cell::RefCell;
use std::ffi::{CString, c_int};
use std::panic::catch_unwind;
use std::ptr::{null_mut};
use std::thread;

use crate::display::desktop::event::Event;
use crate::display::desktop::event::keyboard::{Key, KeyIdentity, KEYCODE_IDENTITY, KeyCodeIdentityList};
use crate::display::desktop::event::window::EventWindow;
use crate::display::desktop::manager::WindowManager;
use crate::display::desktop::property::{WindowProperty, SubWindowOption, WindowPositionOption, get_absolute_position_from_relative, PointerMode};
use crate::display::desktop::window::Window;
use crate::error::StudioError;
use self::cbind::structs::XEvent;
use self::cbind::xkb::{XKB_ALL_COMPONENTS_MASK, XKB_USE_CORE_KBD};

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

macro_rules! key_identity_matchs {
    ($str : ident, $keyid : ident $(,$keysid:ident)*) => {
        match $str.replace("\0", "").as_str() {
            stringify!($keyid) => KeyIdentity::$keyid,
            $(stringify!($keysid) => KeyIdentity::$keysid,)*
            _ => KeyIdentity::NOID,
        }
    };
}

/// Static cache to know if X11 is supported
#[doc(hidden)]
pub static mut X11_SUPPORTED : Option<bool> = Option::None;

pub(crate) struct X11WindowManager<'window> {
    /// Used to fetch X11 events
    pub(crate) x_event : XEvent,  

    /// Event given as reference
    event : Event,

    /// Retained events that will be sent next poll_event 
    pub(crate) retained_events : RefCell<Vec<Event>>,

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

    /// [Window] properties
    pub(super) property : WindowProperty<'window>,

    /// Window has been mapped.
    pub(crate) mapped : bool,

    /// Window is fullscreen.
    pub(crate) fullscreen : bool,   

}

impl<'window> WindowManager<'window> for X11WindowManager<'window> {
    fn new() -> Result<Self, StudioError> {

        unsafe{
            let display = XOpenDisplay(std::ptr::null());      // Display connection
            let atoms = X11Atoms::new(display);                         // X11 Atoms

            match KEYCODE_IDENTITY {    // Initialize Keyboard identity
                Option::None => Self::fill_keycode_identity(display),
                _ => {}
            }
            
            Ok(X11WindowManager {
                x_event: XEvent{ _type:0 }, 
                retained_events: RefCell::new(Vec::new()),
                wm_title: CString::new("").unwrap(), 
                display,
                window: null_mut(),
                atoms,
                event_count: 0,
                property: WindowProperty::new(),
                mapped: false,
                fullscreen: false,
                event: Event::None,
            })
        }
        
    }

    #[inline(always)]
    fn get_window_provider(&self) -> WindowProvider {
        WindowProvider::X11
    }

    #[inline(always)]
    fn poll_event(&mut self) -> &Event  {

        if self.retained_events.borrow().len() > 0 { // Pop event from retained
            self.event = self.retained_events.borrow_mut().pop().unwrap();
        } else {
            // Get count to poll
            if self.event_count == 0 {
                self.sync();
                self.event_count = self.get_event_count();
            }
            self.event = self.get_event();
        }
        
        &self.event
    }

    fn push_event(&self, retain: Event){
        self.retained_events.borrow_mut().push(retain);
    }

    #[inline(always)]
    fn show(&mut self) {
        if !self.property.created {  // Create window if not created
            self.create_window();
        } 
        
        if !self.mapped{
            self.map_window();
        }
    }

    fn recreate(&mut self) {
        todo!()
    }

    fn restore(&mut self) {
        todo!()
    }

    #[inline(always)]
    fn close(&mut self) {
        unsafe {
            XDestroyWindow(self.display, self.window);
            self.property.created = false;
            self.mapped = false;
            self.window = null_mut();   // Delete window pointer.

            // Send closed event to window
            self.push_event(Event::Window(EventWindow::Closed))
        }
    }

    #[inline(always)]
    fn hide(&mut self) {
        unsafe {
            XUnmapWindow(self.display, self.window);
            self.mapped = false;
            self.property.visible = false;
        }
    }

    #[inline(always)]
    fn set_title(&mut self, title : &String) -> bool {
        unsafe {
            self.property.title = title.to_string();
            self.wm_title = CString::from_vec_unchecked(title.as_bytes().to_vec());
            XStoreName(self.display, self.window, self.wm_title.as_ptr() as *mut i8);
        }
        false
    }

    #[inline(always)]
    fn set_position(&mut self, option : WindowPositionOption) -> bool {
        unsafe {
            self.property.relative_position = option.clone();
            self.property.position = get_absolute_position_from_relative(self.property.size, self.property.parent, &option);
            XMoveWindow(self.display, self.window, self.property.position.0, self.property.position.1);
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
            XMoveWindow(self.display, self.window, position.0, position.1);
        }
        false
    }

    #[inline(always)]
    fn show_decoration(&mut self) -> bool {
        self.property.decoration = true;
        true
    }

    #[inline(always)]
    fn hide_decoration(&mut self) -> bool {
        self.property.decoration = false;
        true
    }

    #[inline(always)]
    fn minimize(&mut self) -> bool {
        self.property.minimized = true;
        self.property.maximized = false;
        self.property.fullscreen = Option::None;
        true
    }

    #[inline(always)]
    fn maximize(&mut self) -> bool {
        self.property.minimized = false;
        self.property.maximized = true;
        self.property.fullscreen = Option::None;
        false
    }

    #[inline(always)]
    fn enable_autorepeat(&mut self) -> bool {
        self.property.keyboard.auto_repeat = true;
        false
    }

    #[inline(always)]
    fn disable_autorepeat(&mut self) -> bool {
        self.property.keyboard.auto_repeat = false;
        false
    }

    #[inline(always)]
    fn set_pointer_position(&mut self, position : (i32, i32)) -> bool {
        unsafe {
            XWarpPointer(self.display, self.window, self.window, 0, 0, 
                0, 0, position.0,  position.1);
        }
        false
    }

    #[inline(always)]
    fn show_pointer(&mut self) -> bool {
        unsafe {
            XFixesShowCursor(self.display, self.window);
            self.property.pointer.visible = true;     
        }
        false
    }

    #[inline(always)]
    fn hide_pointer(&mut self) -> bool {
        unsafe {
            self.property.pointer.visible = false;
            XFixesHideCursor(self.display, self.window);
        }
        false
    }

    #[inline(always)]
    fn confine_pointer(&mut self) -> bool {
        unsafe {
            self.property.pointer.confined = true;
            XGrabPointer(self.display, self.window, true, 
            0, GrabModeAsync.try_into().unwrap(), GrabModeAsync.try_into().unwrap(), self.window, 0, CurrentTime);
        }
        false
    }

    #[inline(always)]
    fn release_pointer(&mut self) -> bool {
        unsafe {
            self.property.pointer.confined = false;
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
    fn get_display_handle(&self) -> *const usize {
        self.display as *const usize
    }

    fn get_properties(&self) -> &WindowProperty {
        &self.property
    }

    fn set_parent<'manager: 'window>(&mut self, parent : &'manager Window<'manager>, option : SubWindowOption) -> bool {
        self.property.parent = Some((parent, option));
        false
    }

    fn remove_parent(&mut self) -> bool {
        self.property.parent = Option::None;
        false
    }

    

    fn set_fullscreen(&mut self, fsmode : crate::display::desktop::property::FullScreenMode) -> bool {
        self.property.fullscreen = Some(fsmode);
        true
    }

    fn set_pointer_mode(&mut self, mode : &crate::display::desktop::property::PointerMode) -> bool {
        match mode {
            PointerMode::Acceleration => self.set_pointer_position(self.property.center),
            _ => false,
        };
        self.property.pointer.mode = *mode;
        false
    }

    
}


impl<'window> X11WindowManager<'window> {

    /// Create the window according to window properties.
    #[inline(always)]
    fn create_window(&mut self){
        unsafe {
            // Get root window according to parent.
            let root = match &self.property.parent {
                Some(parent) => parent.0.manager.get_window_handle().unwrap() as *mut u64,
                Option::None => Self::get_x11_default_root_window(self.display),
            };

            self.window = XCreateSimpleWindow(self.display, root, self.property.position.0,self.property.position.1,
                self.property.size.0, self.property.size.1, 0, 0, 0);

            // Set window Type to normal
            x11_change_property!(self.display, self.window, self.atoms, _NET_WM_WINDOW_TYPE, _NET_WM_WINDOW_TYPE_NORMAL);

            // Set window protocols to capture window closing
            XSetWMProtocols(self.display, self.window, &mut self.atoms.WM_DELETE_WINDOW, 1);

            // Allowed actions
            x11_change_property!(self.display, self.window, self.atoms, _NET_WM_ALLOWED_ACTIONS, _NET_WM_ACTION_FULLSCREEN, _NET_WM_ACTION_MINIMIZE, _NET_WM_ACTION_CHANGE_DESKTOP,
                _NET_WM_ACTION_CLOSE, _NET_WM_ACTION_ABOVE, _NET_WM_ACTION_BELOW);

            match &self.property.fullscreen{
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
            self.property.created = true;

            // Send created event to window
            self.push_event(Event::Window(EventWindow::Created))
        }
    }

    /// Map the window according to window properties.
    #[inline(always)]
    fn map_window(&mut self){
        match &self.property.parent{
            Some(parent) => {
                match parent.1 {
                    SubWindowOption::Normal => unsafe { XMapWindow(self.display, self.window) },
                    SubWindowOption::Top => unsafe { XMapRaised(self.display, self.window) },
                    SubWindowOption::Modal =>  {
                        unsafe { XMapRaised(self.display, self.window) }
                        // Send modal showed to parent
                       parent.0.manager.push_event(Event::Window(EventWindow::ModalShowed));
                    },
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



    /// Fill keycode identities link.
    #[inline(always)]
    fn fill_keycode_identity(display : *mut X11Display){
        unsafe {
            let xkb =  XkbGetKeyboard(display, XKB_ALL_COMPONENTS_MASK, XKB_USE_CORE_KBD);
            let mut list: Box<[KeyIdentity;u8::MAX as usize]>  = Box::new([KeyIdentity::NOID;u8::MAX as usize]);
            
            for i in 0..(*xkb).max_key_code {
                //let str = std::ffi::CStr::from_ptr(((*(*(*xkb).names).keys.offset(i as isize)).name).as_ptr());
                let a = (*(*(*xkb).names).keys.offset(i as isize)).name;

                let str = String::from_utf8(a.iter().map(|&c| c as u8).collect()).unwrap();
                let str = str.replace("\0", "");
            
                list[i as usize] = match String::from_utf8(a.iter().map(|&c| c as u8).collect()){
                    Ok(str) => key_identity_matchs!{str, ESC,FK01,FK02,FK03,FK04,FK05,FK06,FK07,FK08,FK09,FK10,FK11,FK12,PRSC,SCLK,PAUS,TLDE,AE01,AE02,AE03,AE04,AE05,AE06,AE07,AE08,AE09,AE10,AE11,AE12,
                        BKSP,TAB,AD01,AD02,AD03,AD04,AD05,AD06,AD07,AD08,AD09,AD10,AD11,AD12,BKSL,CAPS,AC01,AC02,AC03,AC04,AC05,AC06,AC07,AC08,AC09,AC10,AC11,RTRN,LFSH,
                        AB01,AB02,AB03,AB04,AB05,AB06,AB07,AB08,AB09,AB10,RTSH,LCTL,LALT,SPCE,RALT,RCTL,INS,HOME,PGUP,DELE,END,PGDN,UP,LEFT,DOWN,RGHT,NMLK,KPDV,
                        KPMU,KPSU,KPAD,KPEN,KPDL,KP0,KP1,KP2,KP3,KP4,KP5,KP6,KP7,KP8,KP9},
                    Err(_) => KeyIdentity::NOID,
                };
                println!("{} = {:?} = {:?}", i, list[i as usize], str);
            }

            KEYCODE_IDENTITY = Some(KeyCodeIdentityList::new(list));
        }
    }
}

impl<'window> Drop for X11WindowManager<'window> {
    fn drop(&mut self) {
        unsafe {
            // Close display server connection.
            XCloseDisplay(self.display);
        }
    }
}