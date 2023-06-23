use std::cell::RefCell;
use std::ffi::{CString, c_int, CStr, c_void};
use std::panic::catch_unwind;
use std::ptr::{null_mut};
use std::thread;

use crate::display::desktop::event::Event;
use crate::display::desktop::event::window::EventWindow;
use crate::display::desktop::manager::WindowManager;
use crate::display::desktop::manager::linux::x11::cbind::xinput::{XNQueryInputStyle, XIMStyle, XIMPreeditNothing, XIMStatusNothing, XNInputStyle, XNClientWindow, XNFocusWindow};
use crate::display::desktop::property::{WindowProperty, SubWindowOption, WindowPositionOption, get_absolute_position_from_relative, PointerMode, KeyboardMode, WindowEventWaitMode};
use crate::display::desktop::window::Window;
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

/*
macro_rules! key_identity_matchs {
    ($str : ident, $keyid : ident $(,$keysid:ident)*) => {
        match $str.replace("\0", "").replace("-", "D").replace("+", "U").as_str() {
            stringify!($keyid) => KeyIdentity::$keyid,
            $(stringify!($keysid) => KeyIdentity::$keysid,)*
            _ => KeyIdentity::NOID,
        }
    };
}
*/

/// Static cache to know if X11 is supported
#[doc(hidden)]
pub static mut X11_SUPPORTED : Option<bool> = Option::None;

pub(crate) struct X11WindowManager<'window> {
    /// Used to fetch X11 events
    pub(crate) x_event : XEvent,  

    /// Event given as reference
    event : Event,

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
                xim: 0,
                xic: 0,
            })
        }
        
    }

    #[inline(always)]
    fn get_window_provider(&self) -> WindowProvider {
        WindowProvider::X11
    }

    #[inline(always)]
    fn poll_event(&mut self) -> &Event  {

        match self.property.wait_mode {
            WindowEventWaitMode::NeverWait => {
                if self.event == Event::None {
                    self.sync();
                    self.event_count = self.get_event_count();
                }
                self.event = self.fetch_event();
            },
            WindowEventWaitMode::AlwaysWait => {
                self.sync();
                self.event = self.fetch_event();
            },
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
    fn set_event_wait_mode(&mut self, mode : WindowEventWaitMode) -> bool {
        self.property.wait_mode = mode;
        false
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
    fn set_keyboard_mode(&mut self, mode : KeyboardMode) -> bool {
        self.property.keyboard.mode = mode;

        self.set_autorepeat_according_to_kbmode();

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

    #[inline(always)]
    fn get_properties(&self) -> &WindowProperty {
        &self.property
    }

    #[inline(always)]
    fn set_parent<'manager: 'window>(&mut self, parent : &'manager Window<'manager>, option : SubWindowOption) -> bool {
        self.property.parent = Some((parent, option));
        false
    }

    #[inline(always)]
    fn remove_parent(&mut self) -> bool {
        self.property.parent = Option::None;
        false
    }

    
    #[inline(always)]
    fn set_fullscreen(&mut self, fsmode : crate::display::desktop::property::FullScreenMode) -> bool {
        self.property.fullscreen = Some(fsmode);
        true
    }

    #[inline(always)]
    fn set_pointer_mode(&mut self, mode : &crate::display::desktop::property::PointerMode) -> bool {
        match mode {
            PointerMode::Acceleration => self.set_pointer_position(self.property.center),
            _ => false,
        };
        self.property.pointer.mode = *mode;
        false
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

            // Create XIC and XIM
            self.create_xim_xic(self.display, self.window);

            // Set window keyboard mode
            self.set_autorepeat_according_to_kbmode();

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

    /// Set keyboard autorepeat accordig to keyboard mode
    fn set_autorepeat_according_to_kbmode(&self){
        match self.property.keyboard.mode {
            KeyboardMode::DirectInput => unsafe { XAutoRepeatOff(self.display) },  // Deactivate auto repeat
            KeyboardMode::TextInput => unsafe { XAutoRepeatOn(self.display) }, // Activate auto repeat
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

    /*
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
                    Ok(str) => key_identity_matchs!{str, LSGT,TLDE,AE01,AE02,AE03,AE04,AE05,AE06,AE07,AE08,AE09,AE10,AE11,AE12,BKSP,TAB,AD01,AD02,AD03,AD04,AD05,AD06,AD07,AD08,AD09,AD10,AD11,AD12,BKSL,RTRN,CAPS,AC01,AC02,
                        AC03,AC04,AC05,AC06,AC07,AC08,AC09,AC10,AC11,LFSH,AB01,AB02,AB03,AB04,AB05,AB06,AB07,AB08,AB09,AB10,RTSH,LALT,LCTL,SPCE,RCTL,RALT,LWIN,RWIN,COMP,ESC,FK01,FK02,FK03,
                        FK04,FK05,FK06,FK07,FK08,FK09,FK10,FK11,FK12,PRSC,SCLK,PAUS,INS,HOME,PGUP,DELE,END,PGDN,UP,LEFT,DOWN,RGHT,NMLK,KPDV,KPMU,KPSU,KP7,KP8,KP9,KPAD,KP4,KP5,KP6,KP1,KP2,
                        KP3,KPEN,KP0,KPDL,KPEQ,FK13,FK14,FK15,FK16,FK17,FK18,FK19,FK20,FK21,FK22,FK23,FK24,HKTG,AB11,HENK,MUHE,AE13,KATA,HIRA,JPCM,HNGL,HJCV,MUTE,VOLD,VOLU,POWR,STOP,AGAI,
                        PROP,UNDO,FRNT,COPY,OPEN,PAST,FIND,CUT,HELP,LNFD,I120,I126,I128,I129,I147,I148,I149,I150,I151,I152,I153,I154,I155,I156,I157,I158,I159,I160,I161,I162,I163,I164,I165,
                        I166,I167,I168,I169,I170,I171,I172,I173,I174,I175,I176,I177,I178,I179,I180,I181,I182,I183,I184,I185,I186,I187,I188,I189,I190,I208,I209,I210,I211,I212,I213,I214,I215,
                        I216,I217,I218,I219,I220,I221,I222,I223,I224,I225,I226,I227,I228,I229,I230,I231,I232,I233,I234,I235,I236,I237,I238,I239,I240,I241,I242,I243,I244,I245,I246,I247,I248,
                        I249,I250,I251,I252,I253,I254,I255,LVL3,MDSW,ALT,META,SUPR,HYPR},
                    Err(_) => KeyIdentity::NOID,
                };
                println!("{} = {:?} = {:?}", i, list[i as usize], str);
            }

            KEYCODE_IDENTITY = Some(KeyCodeIdentityList::new(list));
        }
    }

    #[allow(non_upper_case_globals)]    // Imported C global aren't formatted according to convention.
    pub fn get_char(key : &Key) -> Option<char> {

        Option::None
        /*
        if key.xic > 0 {    // Make sure Xinput context is initialized.
            let mut buffer:[c_char;4] = [0;4];
            let mut status: c_int = 0;
            //let mut ks: X11Keysim = 0;
            unsafe { Xutf8LookupString(key.xic, key.key_event, &mut buffer as *mut c_char,4, 
                null_mut(), &mut status) };
            
            //let symbol = buffer.
            
            match status {  // Match lookup status
                XBufferOverflow => panic!("Buffer overflow when trying to create keyboard symbol map"),
                XLookupChars => char::from_u32(unsafe { std::mem::transmute::<[c_char; 4], u32>(buffer) }),
                _ => Option::None,
            }

                    
        } else {
            Option::None
        }
        */
        //let keysim = unsafe { XkbKeycodeToKeysym(key.display as *mut u64, key.keycode as u8, 0, 0) };

        //println!("Keysim=`{}`", keysim);
        //char::from_u32(keysim)
        /*
        let mut buffer : [c_uchar;4] = [0;4];
        //let mut buffer:u32 = 0;
        let size = unsafe { xkb_state_key_get_utf8(&key.modifier, key.keycode, buffer.as_mut_ptr(), 4) };

        if size > 0 {   // Character found
            match char::from_u32(u32::from_le_bytes(buffer as [u8;4])){
                Some(char) => Some(char),
                Option::None => Option::None,
            }
        } else {    // No character found
            Option::None
        }
        */

    }
    */
}

impl<'window> Drop for X11WindowManager<'window> {
    fn drop(&mut self) {
        unsafe {
            // Close display server connection.
            XCloseDisplay(self.display);
        }
    }
}