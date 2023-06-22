//! Contains inline event functions.

use std::{ffi::{c_int, c_ulong, c_char, c_void, CStr, c_uchar}, ptr::null_mut};

use crate::display::desktop::{event::{Event, keyboard::{EventKeyboard, KeyModifier, Key}, pointer::{EventPointer, PointerButton}, window::EventWindow}, manager::WindowManager, property::{PointerMode, WindowEventWaitMode, KeyboardMode}};

use super::{ cbind::{structs::{XEvent, Atom}, constants::VisibilityUnobscured, functs::{XGetWindowProperty, XFree, XNextEvent, XEventsQueued, XSync, Xutf8LookupString, XLookupString, XFilterEvent}, xinput::{XBufferOverflow, XLookupChars}}, X11WindowManager};
use super::cbind::{constants::* };


/// Constant value of the window closing message.
pub const WINDOW_CLOSING_MESSAGE_TYPE:u64 = 327;

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

impl<'window> X11WindowManager<'window> {

    /// Get the event queue count
    #[inline(always)]
    pub(crate) fn get_event_count(&self) -> usize {
        unsafe {
            XEventsQueued(self.display, 0).try_into().unwrap()
        }   
    }

    /// Sync X11 window events.
    #[inline(always)]
    pub(crate) fn sync(&self) {
        unsafe {
            XSync(self.display, false);
        }
    }

    /// Get a formatted event according to xevent type
    #[inline(always)]
    #[allow(non_upper_case_globals)]
    pub(super) fn fetch_event(&mut self) -> Event {
        unsafe {

            if self.retained_events.borrow().len() > 0 { // Always pop event from retained first.
               self.retained_events.borrow_mut().pop().unwrap()
            } else {
                match self.property.wait_mode{
                    WindowEventWaitMode::NeverWait => {
                        if self.event_count > 0 {    // If event count > 0, preventing window lock
                            self.event_count -= 1;  // Decrease event count
                            XNextEvent(self.display, &mut self.x_event);
                            let xevent = self.x_event; 
                            self.get_matched_event(&xevent)
                        } else {
                           Event::None
                        }
                    },
                    WindowEventWaitMode::AlwaysWait => {
                        XNextEvent(self.display, &mut self.x_event);    // Will lock window waiting for events
                        let xevent = self.x_event; 
                        self.get_matched_event(&xevent)
                    },
                }
            }                
        }
    }

    /// Get matched event from X11 Event.
    #[inline(always)]
    #[allow(non_upper_case_globals)]
    fn get_matched_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            match xevent._type {
                KeyPress => {
                    match self.property.keyboard.mode { // Interpretation differ according to mode
                        KeyboardMode::DirectInput => self.get_key_down_event(&xevent),
                        KeyboardMode::TextInput => {
                            if !XFilterEvent(xevent, None) {   // Filter some unused keyboard events
                                self.get_key_press_event(xevent)
                            } else {
                                self.fetch_event()
                            }
                        },
                    }
                }
                    
                KeyRelease=> {
                    match self.property.keyboard.mode { // Interpretation differ according to mode
                        KeyboardMode::DirectInput => self.get_key_up_event(&xevent),
                        KeyboardMode::TextInput => self.fetch_event(),  // Release are ignored in TextInput mode
                    }
                    
                },
                ButtonPress=> self.get_button_press_event(&xevent),
                ButtonRelease=> self.get_button_release_event(&xevent),
                MotionNotify=> self.get_motion_notify_event(&xevent),  
                EnterNotify=> self.get_enter_notify_event(&xevent),
                LeaveNotify=> self.get_leave_notify_event(&xevent),
                FocusIn=> self.get_focus_in_event(&xevent),
                FocusOut=> self.get_focus_out_event(&xevent),
                KeymapNotify=> self.get_keymap_notify_event(&xevent),
                Expose=> self.get_expose_event(&xevent),
                GraphicsExpose=> self.get_graphics_expose_event(&xevent),
                NoExpose=> self.get_no_expose_event(&xevent),
                VisibilityNotify=> self.get_visibility_notify_event(&xevent),
                CreateNotify=> self.get_create_notify_event(&xevent),
                DestroyNotify=> self.get_destroy_notify_event(&xevent),
                UnmapNotify=> self.get_unmap_notify_event(&xevent),
                MapNotify=> self.get_map_notify_event(&xevent),
                MapRequest=> self.get_map_request_event(&xevent),
                ReparentNotify=> self.get_reparent_notify_event(&xevent),
                ConfigureNotify=> self.get_configure_notify_event(&xevent),
                ConfigureRequest=> self.get_configure_request_event(&xevent),
                GravityNotify=> self.get_gravity_notify_event(&xevent),
                CirculateNotify=> self.get_circulate_notify_event(&xevent),
                CirculateRequest=> self.get_circulate_request_event(&xevent),
                PropertyNotify=> self.get_property_notify_event(&xevent),
                SelectionClear=> self.get_selection_clear_event(&xevent),
                SelectionRequest=> self.get_selection_request_event(&xevent),
                SelectionNotify=> self.get_selection_notify_event(&xevent),
                ColormapNotify=> self.get_colormap_notify_event(&xevent),
                ClientMessage=> self.get_client_message_event(&xevent),
                MappingNotify=> self.get_mapping_notify_event(&xevent),
                GenericEvent=> self.get_generic_event(&xevent),
                _ => self.get_unknown_event(&xevent),
            }
        }
    }

    /// Get Event created from KeyPress
    #[inline(always)]
    #[allow(non_upper_case_globals)]
    pub(super) fn get_key_press_event(&self, xevent : &XEvent) -> Event {
        unsafe {           
            
            let key : Key =  Key::new(xevent._xkey._state, xevent._xkey._keycode, 
                if self.xic > 0 {    // Make sure Xinput context is initialized.
                    let mut buffer:[c_char;4] = [0;4];
                    let mut status: c_int = 0;

                    Xutf8LookupString(self.xic, &xevent._xkey, &mut buffer as *mut c_char,4, 
                        null_mut(), &mut status); // Get UTF8 character from Xutf8LookupString

                    match status {  // Match lookup status
                        XBufferOverflow => panic!("Buffer overflow when trying to create keyboard symbol map!"),
                        XLookupChars => {
                            match String::from_utf8(vec![buffer[0] as u8, buffer[1] as u8, buffer[2] as u8, buffer[3] as u8]){
                                Ok(s) => s.chars().next(),
                                Err(_) => Option::None,
                            }

                                                        //}
                            
                            
                            //let buffer_u8:[u8;4] = [buffer[0] as u8, buffer[1] as u8, buffer[2] as u8, buffer[3] as u8];

                            //match String::from_utf8(Vec::fr){

                            //}
                            //char::from_u32(u32::from_ne_bytes(buffer_u8))
                           
                            /*
                            let a = u32::from_ne_bytes(buffer_u8);
                            let c = char::from_u32(i)

                            let mut vec : Vec<u8> = Vec::new();
                            vec.push(buffer[0] as u8);
                            vec.push(buffer[1] as u8);
                                vec.push(buffer[2] as u8);
                                    vec.push(buffer[3] as u8);
                            match String::from_utf8(vec){
                                Ok(s) => println!("XIC Got `{}`", s),
                                Err(err) => println!("XIC Err `{}`", err),
                            }
                            */
                        },
                        _ => Option::None,  // No char associated
                    }

                } else {
                    Option::None    // No char associated
                });
            
                /*
                match status {  // Match lookup status
                    XBufferOverflow => panic!("Buffer overflow when trying to create keyboard symbol map"),
                    XLookupChars => char::from_u32(unsafe { std::mem::transmute::<[c_char; 4], u32>(buffer) }),
                    _ => Option::None,
                }
                */
    
                        
                

            Event::Keyboard(EventKeyboard::KeyPress(key))
        }
    }

    /// Get Event created from keydown
    #[inline(always)]
    pub(super) fn get_key_down_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            Event::Keyboard(EventKeyboard::KeyDown(xevent._xkey._keycode))            
        }
    }


    /// Get Event created from KeyRelease with anti-repeat protection.
    /// Steps :
    /// 1. Peek next event
    /// 2. if is KeyDown same key, ignore both events.
    /// 3. else, return Keyup and retain peeked event.
    #[inline(always)]
    pub fn get_key_up_event(&mut self, xevent : &XEvent) -> Event{
        unsafe {
            if self.event_count > 0 {
                // 1. Peek next event
                let peeked = self.fetch_event();

                // 2. Make sure it's keyboard event
                if let Event::Keyboard(kb_event) = peeked {
                    // 3. Make sure it's keydown event
                    if let EventKeyboard::KeyDown(keycode) = kb_event {
                        // 4. If same keycode, ignore both event and get next
                        if keycode == xevent._xkey._keycode {   
                            return self.fetch_event();
                        }
                    }
                }

                // 5. If not ignored, retain peeked event
                self.push_event(peeked);
            } 
            
            // Key is not repeating, return current keyup
            Event::Keyboard(EventKeyboard::KeyUp(xevent._xkey._keycode))
        }
    }

/// Get modifier from Xkey state
    /// 
    /// Reference(s)
    /// <https://github.com/glfw/glfw/blob/7e8da57094281c73a0be5669a4b79686b4917f6c/src/x11_window.c#L186>
    #[inline(always)]
    fn get_key_modifier_from_state(state : u32) -> u8 {

        let mut modifier : u8 = 0;

        if state & ShiftMask as u32 > 0 {   // Is SHIFT modifier on?
            modifier = modifier | KeyModifier::SHIFT;
        }
            
        if state & ControlMask as u32 > 0 { // Is CTRL modifier on?
            modifier = modifier | KeyModifier::CTRL;
        }

        if state & Mod1Mask as u32 > 0 {    // Is ALT modifier on?
            modifier = modifier | KeyModifier::ALT;
        }

        if state & Mod3Mask as u32 > 0 {    // Is META modifier on?
            modifier = modifier | KeyModifier::META;
        }

        if state & Mod4Mask as u32 > 0 {    // Is COMMAND/SUPER modifier on?
            modifier = modifier | KeyModifier::COMMAND;
        }

        if state & Mod5Mask as u32 > 0 {    // Is HYPER modifier on?
            modifier = modifier | KeyModifier::HYPER;
        }

        if state & LockMask as u32 > 0 {    // Is CAPSLOCK on?
            modifier = modifier | KeyModifier::CAPSLOCK;
        }

        if state & Mod2Mask as u32 > 0 {    // Is NUMLOCK on?
            modifier = modifier | KeyModifier::NUMLOCK;
        }

        modifier
    }

    /// Get Event created from ButtonPress
    /// Mouse button press.
    #[inline(always)]
    pub(super) fn get_button_press_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            Event::Pointer(EventPointer::ButtonDown( match xevent._xbutton._button {
                POINTER_LEFT_BUTTON => PointerButton::LeftButton,
                POINTER_MIDDLE_BUTTON => PointerButton::MiddleButton,
                POINTER_RIGHT_BUTTON => PointerButton::RightButton,
                POINTER_PREVIOUS_BUTTON => PointerButton::PreviousButton,
                POINTER_NEXT_BUTTON => PointerButton::NextButton,
                POINTER_SCROLL_UP => PointerButton::ScrollUp,
                POINTER_SCROLL_DOWN => PointerButton::ScrollDown,
                POINTER_SCROLL_LEFT => PointerButton::ScrollLeft,
                POINTER_SCROLL_RIGHT => PointerButton::ScrollRight,
                _ => PointerButton::Other(xevent._xbutton._button.try_into().unwrap()),
            }, (xevent._xbutton._x, xevent._xbutton._y)))
        }
    }

    /// Get Event created from ButtonRelease
    /// Mouse button release.
    #[inline(always)]
    pub(super) fn get_button_release_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            Event::Pointer(EventPointer::ButtonUp( match xevent._xbutton._button {
                POINTER_LEFT_BUTTON => PointerButton::LeftButton,
                POINTER_MIDDLE_BUTTON => PointerButton::MiddleButton,
                POINTER_RIGHT_BUTTON => PointerButton::RightButton,
                POINTER_PREVIOUS_BUTTON => PointerButton::PreviousButton,
                POINTER_NEXT_BUTTON => PointerButton::NextButton,
                POINTER_SCROLL_UP => PointerButton::ScrollUp,
                POINTER_SCROLL_DOWN => PointerButton::ScrollDown,
                POINTER_SCROLL_LEFT => PointerButton::ScrollLeft,
                POINTER_SCROLL_RIGHT => PointerButton::ScrollRight,
                _ => PointerButton::Other(xevent._xbutton._button.try_into().unwrap()),
            }, (xevent._xbutton._x, xevent._xbutton._y)))
        }
    }

    /// Get Event created from MotionNotify.
    /// Happens when pointer is moving over window
    #[inline(always)]
    pub(super) fn get_motion_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            match self.property.pointer.mode{
                PointerMode::Cursor => Event::Pointer(EventPointer::Moved((xevent._xmotion._x, xevent._xmotion._y))),
                PointerMode::Acceleration => {
                    // Calc delta acceleration
                    let acceleration = (xevent._xmotion._x - self.property.center.0, 
                        xevent._xmotion._y - self.property.center.1);

                    if acceleration.0 != 0 && acceleration.1 != 0 { // Send acceleration only if it moved.
                        // Reset pointer to center
                        self.set_pointer_position(self.property.center);

                        // Send acceleration event.
                        Event::Pointer(EventPointer::Acceleration(acceleration))
                    } else {
                        self.fetch_event()   // Ignore and poll next event
                    }
                },     
            }
        }
    }

    /// Get Event created from EnterNotify.
    /// Pointer entered window
    #[inline(always)]
    pub(super) fn get_enter_notify_event(&mut self, _xevent : &XEvent) -> Event {
        // Hide cursor if supposed to be hidden.
        if !self.property.pointer.visible {
            self.property.pointer.visible = true;
            self.hide_pointer();
        }

        Event::Window(EventWindow::CursorEnter)
    }

    /// Get Event created from LeaveNotify
    /// Pointer left window
    #[inline(always)]
    pub(super) fn get_leave_notify_event(&mut self, _xevent : &XEvent) -> Event {
         // Show hidden cursor when out of window.
         if !self.property.pointer.visible {
            self.show_pointer();
            self.property.pointer.visible = false;    // Tell pointer it is still hidden
        }

        Event::Window(EventWindow::CursorLeave)
    }

    /// Get Event created from FocusIn
    /// Window got focus.
    #[inline(always)]
    pub(super) fn get_focus_in_event(&mut self, _xevent : &XEvent) -> Event {
        // If cursor is confined, confine cursor on focus.
        if self.property.pointer.confined {
            self.confine_pointer();
        }

        Event::Window(EventWindow::Focus)
    }

    /// Get Event created from FocusOut
    /// Window lost focus
    #[inline(always)]
    pub(super) fn get_focus_out_event(&mut self, _xevent : &XEvent) -> Event {
        // If cursor is confined, confine cursor on focus.
        if self.property.pointer.confined {
            self.release_pointer();
            self.property.pointer.confined = true;    // Tell pointer it is still confined
        }

        Event::Window(EventWindow::Blur)
    }

    /// Get Event created from KeymapNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_keymap_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), KeymapNotify({})", self, xevent._type); 
            self.fetch_event()
        }
    }

    /// Get Event created from Expose
    /// Part of window need to be redrawed 
    #[inline(always)]
    pub(super) fn get_expose_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            Event::Window(EventWindow::Exposed((xevent._xexpose._x, xevent._xexpose._y), (xevent._xexpose._width as u32, xevent._xexpose._height as u32)))
        }
    }

    /// Get Event created from GraphicsExpose
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_graphics_expose_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), GraphicsExpose({})", self, xevent._type); 
            self.fetch_event()
        }
    }

    /// Get Event created from NoExpose
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_no_expose_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), NoExpose({})", self, xevent._type);
            self.fetch_event()
        }
    }

    /// Get Event created from VisibilityNotify
    /// Window visibility changed
    #[inline(always)]
    pub(super) fn get_visibility_notify_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            if xevent._xvisibility._state == VisibilityUnobscured {
                Event::Window(EventWindow::Shown)
            } else {
                Event::Window(EventWindow::Hidden)
            }
        }
    }

    /// Get Event created from CreateNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_create_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), CreateNotify({})", self, xevent._type);
            self.fetch_event()
        }
    }

    /// Get Event created from DestroyNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_destroy_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), DestroyNotify({})", self, xevent._type);
            self.fetch_event()
        }
    }

    /// Get Event created from UnmapNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_unmap_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), UnmapNotify({})", self, xevent._type);
            self.fetch_event()
        }
    }

    /// Get Event created from MapNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_map_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), MapNotify({})", self, xevent._type);
            self.fetch_event()
        }
    }

    /// Get Event created from MapRequest
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_map_request_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), MapRequest({})", self, xevent._type);
            self.fetch_event()
        }
    }

    /// Get Event created from ReparentNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_reparent_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), ReparentNotify({})", self, xevent._type);
            self.fetch_event()
        }
    }

    /// Get Event created from ConfigureNotify
    /// Window position and/or size changed
    #[inline(always)]
    pub(super) fn get_configure_notify_event(&mut self, _xevent : &XEvent) -> Event {
        unsafe {
            let position = (self.x_event._xconfigure._x, self.x_event._xconfigure._y);
            let size = (self.x_event._xconfigure._width as u32, self.x_event._xconfigure._height as u32);
        
            // By default, set event as none.
            let mut event = Event::None;

            if position != self.property.position && size != self.property.size {
                event = Event::Window(EventWindow::MovedResized(position, size));
            } else if position != self.property.position {
                event = Event::Window(EventWindow::Moved(position));
            } else if size != self.property.size  {
                event = Event::Window(EventWindow::Resized(size));
            }

            self.property.position = position;
            self.property.size = size;

            event
        }
    }

    /// Get Event created from ConfigureRequest
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_configure_request_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), ConfigureRequest({})", self, xevent._type); 
            self.fetch_event()
        }
    }

    /// Get Event created from GravityNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_gravity_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), GravityNotify({})", self, xevent._type); 
            self.fetch_event()
        }
    }

    /// Get Event created from CirculateNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_circulate_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), CirculateNotify({})", self, xevent._type); 
            self.fetch_event()
        }
    }

    /// Get Event created from CirculateRequest
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_circulate_request_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), CirculateRequest({})", self, xevent._type); 
            self.fetch_event()
        }
    }

    /// Get Event created from PropertyNotify
    /// X11 Window atom property changed
    #[inline(always)]
    pub(super) fn get_property_notify_event(&mut self, _xevent : &XEvent) -> Event {
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
                                // Ignore unknown state name
                                _state => { 
                                    //println!("State={:?}", CStr::from_ptr(XGetAtomName(self.display, state)).to_str().unwrap());
                                }
                            }
                        }
                    },

                    // Anything else is an error
                    _ => panic!("Wrong `actual_format_return` format size!"),
                }
            }

            // Free data returned.
            XFree(prop_return as *mut c_void);

            // Event to return
            let mut event = Event::None;

            // Return event. By priority > Fullscreen > Minimized > Maximized > Restored > None
            if fullscreen {   // Send fullscreen if not already registered.
                if !self.fullscreen {
                    event = Event::Window(EventWindow::Fullscreen);
                }
            } else if hidden {   // Send minimized if not already registered.
                    if !self.property.minimized {
                        event = Event::Window(EventWindow::Minimized);
                    }
            } else if maximized {   // Send maximized if not already registered.
                if !self.property.maximized {
                    event = Event::Window(EventWindow::Maximized);
                }
            } else {    // Send restore if not already registered.
                if self.fullscreen != fullscreen || 
                    self.property.maximized != maximized || 
                    self.property.minimized != hidden {
                        event = Event::Window(EventWindow::Restored);
                    }
            }

            // Update window properties
            self.fullscreen = fullscreen;
            self.property.maximized = maximized;
            self.property.minimized = hidden;

            event
        }
    }

    /// Get Event created from SelectionClear
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_selection_clear_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), SelectionClear({})", self, xevent._type); 
            self.fetch_event()
        }
    }

    /// Get Event created from SelectionRequest
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_selection_request_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), SelectionRequest({})", self, xevent._type); 
            self.fetch_event()
        }
    }

    /// Get Event created from SelectionNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_selection_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), SelectionNotify({})", self, xevent._type); 
            self.fetch_event()
        }
    }

    /// Get Event created from ColormapNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_colormap_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), ColormapNotify({})", self, xevent._type);
            self.fetch_event()
        }
    }

    /// Get Event created from ClientMessage
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_client_message_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            match xevent._xclient._message_type {
                WINDOW_CLOSING_MESSAGE_TYPE => Event::Window(EventWindow::CloseRequest),
                _ => {
                    #[cfg(debug_assertions)]
                    println!("Unknown ClientMessage({:p}), Type({})", self, xevent._xclient._message_type);
                    self.fetch_event()
                },
            }
        }
    }

    /// Get Event created from MappingNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_mapping_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), MappingNotify({})", self, xevent._type); 
            self.fetch_event()
        }
    }

    /// Get Event created from GenericEvent
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_generic_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), GenericEvent({})", self, xevent._type); 
            self.fetch_event()
        }
    }

    /// Get Event created from unknown
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_unknown_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), _({})", self, xevent._type); 
            self.fetch_event()
        }
    }

}
