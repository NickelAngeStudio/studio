//! Contains inline event functions.

use std::{ffi::{c_int, c_ulong, c_char, CStr}, ptr::null_mut};

use crate::display::desktop::{event::{Event, EventKeyboard, EventMouse, EventWindow}, pointer::PointerMode, provider::linux::x11::cbind::functs::XGetAtomName, window::Window};

use super::{X11Window, cbind::{structs::{XEvent, Atom}, constants::VisibilityUnobscured, functs::{XGetWindowProperty, XFree, XNextEvent}}};
use super::cbind::{constants::* };


/// Constant value of the window closing message.
pub const WINDOW_CLOSING_MESSAGE_TYPE:u64 = 327;

impl X11Window {

    /// Get a formatted event according to xevent type
    #[inline(always)]
    #[allow(non_upper_case_globals)]
    pub(super) fn get_event(&mut self) -> Event {
        unsafe {
            // Only if we have something to poll
            if self.event_count > 0 {
                self.event_count -= 1;  // Decrease event count

                if self.retained_events.len() > 0 {
                    self.retained_events.pop().unwrap() // Pop event from retained
                } else {
                    XNextEvent(self.display, &mut self.x_event);
                    let xevent = self.x_event; 
                    
                    match xevent._type {
                        KeyPress => self.get_key_press_event(&xevent),
                        KeyRelease=> self.get_key_release_event(&xevent),
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
            } else {
                Event::None   // Return None event
            }
        }
    }

    /// Retain an event for next fetch.
    #[inline(always)]
    fn retain_event(&mut self, retain: Event) {
        self.retained_events.push(retain);
        self.event_count += 1;
    }
    

    /// Get Event created from KeyPress
    #[inline(always)]
    pub(super) fn get_key_press_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            Event::Keyboard(EventKeyboard::KeyDown(xevent._xkey._keycode))
        }
    }

    /// Get Event created from KeyRelease.
    #[inline(always)]
    pub(super) fn get_key_release_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {

            if self.keyboard.auto_repeat {  // No anti-repeat routine
                Event::Keyboard(EventKeyboard::KeyUp(xevent._xkey._keycode))
            } else {    // Use anti-repeat routine
                self.get_anti_repeat_key_release_event(xevent)
            }
            
        }
    }

    /// Get Event created from KeyRelease with anti-repeat protection.
    /// Steps :
    /// 1. Peek next event
    /// 2. if is KeyDown same key, ignore both events.
    /// 3. else, return Keyup and retain peeked event.
    #[inline(always)]
    pub fn get_anti_repeat_key_release_event(&mut self, xevent : &XEvent) -> Event{
        unsafe {
            if self.event_count > 0 {
                // 1. Peek next event
                let peeked = self.get_event();

                // 2. Make sure it's keyboard event
                if let Event::Keyboard(kb_event) = peeked {
                    // 3. Make sure it's keydown event
                    if let EventKeyboard::KeyDown(keycode) = kb_event {
                        // 4. If same keycode, ignore both event and get next
                        if keycode == xevent._xkey._keycode {   
                            return self.get_event();
                        }
                    }
                }

                // 5. If not ignored, retain peeked event
                self.retain_event(peeked);
            } 
            
            // Key is not repeating, return current keyup
            Event::Keyboard(EventKeyboard::KeyUp(xevent._xkey._keycode))
        }
    }

    /// Get Event created from ButtonPress
    /// Mouse button press.
    #[inline(always)]
    pub(super) fn get_button_press_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("EventMouse::ButtonDown({})", xevent._xbutton._button); 

            Event::Mouse(EventMouse::ButtonDown(xevent._xbutton._button.try_into().unwrap() , (xevent._xbutton._x, xevent._xbutton._y)))
        }
    }

    /// Get Event created from ButtonRelease
    /// Mouse button release.
    #[inline(always)]
    pub(super) fn get_button_release_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("EventMouse::ButtonUp({})", xevent._xbutton._button); 

            Event::Mouse(EventMouse::ButtonUp(xevent._xbutton._button.try_into().unwrap() , (xevent._xbutton._x, xevent._xbutton._y)))
        }
    }

    /// Get Event created from MotionNotify.
    /// Happens when pointer is moving over window
    #[inline(always)]
    pub(super) fn get_motion_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            match self.pointer.mode {   
                PointerMode::Pointer => {
                    self.pointer.position = (xevent._xmotion._x, xevent._xmotion._y);
                    Event::Mouse(EventMouse::Moved((xevent._xmotion._x, xevent._xmotion._y)))
                },
                PointerMode::Acceleration => {
                    let position = (xevent._xmotion._x - self.property.center.0, 
                        xevent._xmotion._y - self.property.center.1);
                    // Report acceleration only if movement occurred
                    if position.0 != 0 || position.1 != 0 {
                        // Re-center pointer
                        self.set_pointer_position(self.property.center);

                        // Return position
                        Event::Mouse(EventMouse::Moved(position))
                    } else {
                        self.poll_event()
                    }
                }
            }
        }
    }

    /// Get Event created from EnterNotify.
    /// Pointer entered window
    #[inline(always)]
    pub(super) fn get_enter_notify_event(&mut self, _xevent : &XEvent) -> Event {
        // Hide cursor if supposed to be hidden.
        if !self.pointer.is_visible {
            self.pointer.is_visible = true;
            self.hide_pointer();
        }

        Event::Window(EventWindow::CursorEnter())
    }

    /// Get Event created from LeaveNotify
    /// Pointer left window
    #[inline(always)]
    pub(super) fn get_leave_notify_event(&mut self, _xevent : &XEvent) -> Event {
         // Show hidden cursor when out of window.
         if !self.pointer.is_visible {
            self.show_pointer();
            self.pointer.is_visible = false;    // Tell pointer it is still hidden
        }

        Event::Window(EventWindow::CursorLeave())
    }

    /// Get Event created from FocusIn
    /// Window got focus.
    #[inline(always)]
    pub(super) fn get_focus_in_event(&mut self, _xevent : &XEvent) -> Event {
        // If cursor is confined, confine cursor on focus.
        if self.pointer.is_confined {
            self.confine_pointer();
        }

        Event::Window(EventWindow::Focus())
    }

    /// Get Event created from FocusOut
    /// Window lost focus
    #[inline(always)]
    pub(super) fn get_focus_out_event(&mut self, _xevent : &XEvent) -> Event {
        // If cursor is confined, confine cursor on focus.
        if self.pointer.is_confined {
            self.release_pointer();
            self.pointer.is_confined = true;    // Tell pointer it is still confined
        }

        Event::Window(EventWindow::Blur())
    }

    /// Get Event created from KeymapNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_keymap_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), KeymapNotify({})", self, xevent._type); 
            self.get_event()
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
            self.get_event()
        }
    }

    /// Get Event created from NoExpose
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_no_expose_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), NoExpose({})", self, xevent._type);
            self.get_event()
        }
    }

    /// Get Event created from VisibilityNotify
    /// Window visibility changed
    #[inline(always)]
    pub(super) fn get_visibility_notify_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            if xevent._xvisibility._state == VisibilityUnobscured {
                Event::Window(EventWindow::Shown())
            } else {
                Event::Window(EventWindow::Hidden())
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
            self.get_event()
        }
    }

    /// Get Event created from DestroyNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_destroy_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), DestroyNotify({})", self, xevent._type);
            self.get_event()
        }
    }

    /// Get Event created from UnmapNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_unmap_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), UnmapNotify({})", self, xevent._type);
            self.get_event()
        }
    }

    /// Get Event created from MapNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_map_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), MapNotify({})", self, xevent._type);
            self.get_event()
        }
    }

    /// Get Event created from MapRequest
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_map_request_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), MapRequest({})", self, xevent._type);
            self.get_event()
        }
    }

    /// Get Event created from ReparentNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_reparent_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), ReparentNotify({})", self, xevent._type);
            self.get_event()
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

            // Update window properties
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
            self.get_event()
        }
    }

    /// Get Event created from GravityNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_gravity_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), GravityNotify({})", self, xevent._type); 
            self.get_event()
        }
    }

    /// Get Event created from CirculateNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_circulate_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), CirculateNotify({})", self, xevent._type); 
            self.get_event()
        }
    }

    /// Get Event created from CirculateRequest
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_circulate_request_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), CirculateRequest({})", self, xevent._type); 
            self.get_event()
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
            XFree(prop_return);

            // Event to return
            let mut event = Event::None;

            // Return event. By priority > Fullscreen > Minimized > Maximized > Restored > None
            if fullscreen {   // Send fullscreen if not already registered.
                if !self.property.is_fullscreen {
                    event = Event::Window(EventWindow::Fullscreen());
                }
            } else if hidden {   // Send minimized if not already registered.
                    if !self.property.is_minimized {
                        event = Event::Window(EventWindow::Minimized());
                    }
            } else if maximized {   // Send maximized if not already registered.
                if !self.property.is_maximized {
                    event = Event::Window(EventWindow::Maximized());
                }
            } else {    // Send restore if not already registered.
                if self.property.is_fullscreen != fullscreen || 
                    self.property.is_maximized != maximized || 
                    self.property.is_minimized != hidden {
                        event = Event::Window(EventWindow::Restored());
                    }
            }

            // Update window properties
            self.property.is_fullscreen = fullscreen;
            self.property.is_maximized = maximized;
            self.property.is_minimized = hidden;

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
            self.get_event()
        }
    }

    /// Get Event created from SelectionRequest
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_selection_request_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), SelectionRequest({})", self, xevent._type); 
            self.get_event()
        }
    }

    /// Get Event created from SelectionNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_selection_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), SelectionNotify({})", self, xevent._type); 
            self.get_event()
        }
    }

    /// Get Event created from ColormapNotify
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_colormap_notify_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), ColormapNotify({})", self, xevent._type);
            self.get_event()
        }
    }

    /// Get Event created from ClientMessage
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_client_message_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            match xevent._xclient._message_type {
                WINDOW_CLOSING_MESSAGE_TYPE => Event::Window(EventWindow::Close()),
                _ => {
                    #[cfg(debug_assertions)]
                    println!("Unknown ClientMessage({:p}), Type({})", self, xevent._xclient._message_type);
                    self.get_event()
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
            self.get_event()
        }
    }

    /// Get Event created from GenericEvent
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_generic_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), GenericEvent({})", self, xevent._type); 
            self.get_event()
        }
    }

    /// Get Event created from unknown
    /// Unknown use for now.
    #[inline(always)]
    pub(super) fn get_unknown_event(&mut self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), _({})", self, xevent._type); 
            self.get_event()
        }
    }

}
