//! Contains inline event functions.

use std::{ffi::{c_int, c_ulong, c_char, CStr}, ptr::null_mut};

use crate::display::desktop::{event::{Event, EventKeyboard, EventMouse, EventWindow}, pointer::PointerMode, provider::linux::x11::cbind::functs::XGetAtomName};

use super::{X11Window, cbind::{structs::{XEvent, Atom}, constants::VisibilityUnobscured, functs::{XGetWindowProperty, XFree}}};

impl X11Window {

    /// Get Event created from KeyPress
    #[inline(always)]
    pub fn get_key_press_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            Event::Keyboard(EventKeyboard::KeyDown(xevent._xkey._keycode))
        }
    }

    /// Get Event created from KeyRelease
    #[inline(always)]
    pub fn get_key_release_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            Event::Keyboard(EventKeyboard::KeyUp(xevent._xkey._keycode))
        }
    }

    /// Get Event created from ButtonPress
    /// Mouse button press.
    #[inline(always)]
    pub fn get_button_press_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            Event::Mouse(EventMouse::ButtonDown(xevent._xbutton._button.try_into().unwrap() , (xevent._xbutton._x, xevent._xbutton._y)))
        }
    }

    /// Get Event created from ButtonRelease
    /// Mouse button release.
    #[inline(always)]
    pub fn get_button_release_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            Event::Mouse(EventMouse::ButtonUp(xevent._xbutton._button.try_into().unwrap() , (xevent._xbutton._x, xevent._xbutton._y)))
        }
    }

    /// Get Event created from MotionNotify (cursor moved)
    #[inline(always)]
    pub fn get_motion_notify_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            match self.property.cursor.mode {   
                PointerMode::Pointer => Event::Mouse(EventMouse::Moved((xevent._xmotion._x, xevent._xmotion._y))),
                PointerMode::Acceleration => {
                    let position = (xevent._xmotion._x - self.property.center.0, 
                        xevent._xmotion._y - self.property.center.1);
                    // Report acceleration only if movement occurred
                    if position.0 != 0 || position.1 != 0 {
                        Event::Mouse(EventMouse::Moved(position))
                    } else {
                        Event::None
                    }
                }
            }
        }
    }

    /// Get Event created from EnterNotify.
    /// Cursor entered window
    #[inline(always)]
    pub fn get_enter_notify_event(&self, _xevent : &XEvent) -> Event {
        Event::Window(EventWindow::CursorEnter())
    }

    /// Get Event created from LeaveNotify
    /// Cursor left window
    #[inline(always)]
    pub fn get_leave_notify_event(&self, _xevent : &XEvent) -> Event {
        Event::Window(EventWindow::CursorLeave())
    }

    /// Get Event created from FocusIn
    /// Window got focus.
    #[inline(always)]
    pub fn get_focus_in_event(&self, _xevent : &XEvent) -> Event {
        Event::Window(EventWindow::Focus())
    }

    /// Get Event created from FocusOut
    /// Window lost focus
    #[inline(always)]
    pub fn get_focus_out_event(&self, _xevent : &XEvent) -> Event {
        Event::Window(EventWindow::Blur())
    }

    /// Get Event created from KeymapNotify
    /// Unknown use for now.
    #[inline(always)]
    pub fn get_keymap_notify_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), KeymapNotify({})", self, xevent._type); 
            Event::Unknown
        }
    }

    /// Get Event created from Expose
    /// Part of window need to be redrawed 
    #[inline(always)]
    pub fn get_expose_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            Event::Window(EventWindow::Exposed((xevent._xexpose._x, xevent._xexpose._y), (xevent._xexpose._width as u32, xevent._xexpose._height as u32)))
        }
    }

    /// Get Event created from GraphicsExpose
    /// Unknown use for now.
    #[inline(always)]
    pub fn get_graphics_expose_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), GraphicsExpose({})", self, xevent._type); 
            Event::Unknown
        }
    }

    /// Get Event created from NoExpose
    /// Unknown use for now.
    #[inline(always)]
    pub fn get_no_expose_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), NoExpose({})", self, xevent._type);
            Event::Unknown
        }
    }

    /// Get Event created from VisibilityNotify
    /// Window visibility changed
    #[inline(always)]
    pub fn get_visibility_notify_event(&self, xevent : &XEvent) -> Event {
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
    pub fn get_create_notify_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), CreateNotify({})", self, xevent._type);
            Event::Unknown
        }
    }

    /// Get Event created from DestroyNotify
    /// Unknown use for now.
    #[inline(always)]
    pub fn get_destroy_notify_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), DestroyNotify({})", self, xevent._type);
            Event::Unknown
        }
    }

    /// Get Event created from UnmapNotify
    /// Unknown use for now.
    #[inline(always)]
    pub fn get_unmap_notify_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), UnmapNotify({})", self, xevent._type);
            Event::Unknown
        }
    }

    /// Get Event created from MapNotify
    /// Unknown use for now.
    #[inline(always)]
    pub fn get_map_notify_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), MapNotify({})", self, xevent._type);
            Event::Unknown
        }
    }

    /// Get Event created from MapRequest
    /// Unknown use for now.
    #[inline(always)]
    pub fn get_map_request_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), MapRequest({})", self, xevent._type);
            Event::Unknown
        }
    }

    /// Get Event created from ReparentNotify
    /// Unknown use for now.
    #[inline(always)]
    pub fn get_reparent_notify_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), ReparentNotify({})", self, xevent._type);
            Event::Unknown
        }
    }

    /// Get Event created from ConfigureNotify
    /// Window position and/or size changed
    #[inline(always)]
    pub fn get_configure_notify_event(&mut self, _xevent : &XEvent) -> Event {
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
    pub fn get_configure_request_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), ConfigureRequest({})", self, xevent._type); 
            Event::Unknown
        }
    }

    /// Get Event created from GravityNotify
    /// Unknown use for now.
    #[inline(always)]
    pub fn get_gravity_notify_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), GravityNotify({})", self, xevent._type); 
            Event::Unknown
        }
    }

    /// Get Event created from CirculateNotify
    /// Unknown use for now.
    #[inline(always)]
    pub fn get_circulate_notify_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), CirculateNotify({})", self, xevent._type); 
            Event::Unknown
        }
    }

    /// Get Event created from CirculateRequest
    /// Unknown use for now.
    #[inline(always)]
    pub fn get_circulate_request_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), CirculateRequest({})", self, xevent._type); 
            Event::Unknown
        }
    }

    /// Get Event created from PropertyNotify
    /// X11 Window atom property changed
    #[inline(always)]
    pub fn get_property_notify_event(&mut self, _xevent : &XEvent) -> Event {
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
    pub fn get_selection_clear_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), SelectionClear({})", self, xevent._type); 
            Event::Unknown
        }
    }

    /// Get Event created from SelectionRequest
    /// Unknown use for now.
    #[inline(always)]
    pub fn get_selection_request_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), SelectionRequest({})", self, xevent._type); 
            Event::Unknown
        }
    }

    /// Get Event created from SelectionNotify
    /// Unknown use for now.
    #[inline(always)]
    pub fn get_selection_notify_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), SelectionNotify({})", self, xevent._type); 
            Event::Unknown
        }
    }

    /// Get Event created from ColormapNotify
    /// Unknown use for now.
    #[inline(always)]
    pub fn get_colormap_notify_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), ColormapNotify({})", self, xevent._type);
            Event::Unknown
        }
    }

    /// Get Event created from ClientMessage
    /// Unknown use for now.
    #[inline(always)]
    pub fn get_client_message_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), ClientMessage({})", self, xevent._type); 
            Event::Unknown
        }
    }

    /// Get Event created from MappingNotify
    /// Unknown use for now.
    #[inline(always)]
    pub fn get_mapping_notify_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), MappingNotify({})", self, xevent._type); 
            Event::Unknown
        }
    }

    /// Get Event created from GenericEvent
    /// Unknown use for now.
    #[inline(always)]
    pub fn get_generic_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), GenericEvent({})", self, xevent._type); 
            Event::Unknown
        }
    }

    /// Get Event created from unknown
    /// Unknown use for now.
    #[inline(always)]
    pub fn get_unknown_event(&self, xevent : &XEvent) -> Event {
        unsafe {
            #[cfg(debug_assertions)]
            println!("Display({:p}), _({})", self, xevent._type); 
            Event::Unknown
        }
    }


    /*

                    ButtonPress=> { println!("Display({:p}), ButtonPress({})", self, xevent._type); Event::Unknown },
                    ButtonRelease=> { println!("Display({:p}), ButtonRelease({})", self, xevent._type); Event::Unknown },

                    // Cursor moved
                    MotionNotify=> {    
                        match self.property.cursor.mode {   
                            PointerMode::Pointer => Event::Mouse(EventMouse::Moved((xevent._xmotion._x, xevent._xmotion._y))),
                            PointerMode::Acceleration => {
                                let position = (xevent._xmotion._x - self.property.center.0, 
                                    xevent._xmotion._y - self.property.center.1);
                                // Report acceleration only if movement occurred
                                if position.0 != 0 || position.1 != 0 {
                                    Event::Mouse(EventMouse::Moved(position))
                                } else {
                                    Event::None
                                }
                            }
                        }
                    },

                    // Cursor entered window
                    EnterNotify=> Event::Window(EventWindow::CursorEnter()),

                    // Cursor left window
                    LeaveNotify=> Event::Window(EventWindow::CursorLeave()),

                    // Window got focus
                    FocusIn=> Event::Window(EventWindow::Focus()),

                    // Window lost focus
                    FocusOut=> Event::Window(EventWindow::Blur()),

                    KeymapNotify=> { println!("Display({:p}), KeymapNotify({})", self, xevent._type); Event::Unknown },

                    // Part of window need to be redrawed 
                    Expose=> { 
                        Event::Window(EventWindow::Exposed((xevent._xexpose._x, xevent._xexpose._y), (xevent._xexpose._width as u32, xevent._xexpose._height as u32)))
                    },
                    GraphicsExpose=> { println!("Display({:p}), GraphicsExpose({})", self, xevent._type); Event::Unknown },
                    NoExpose=> { println!("Display({:p}), NoExpose({})", self, xevent._type); Event::Unknown },
                    VisibilityNotify=> { 
                        if xevent._xvisibility._state == VisibilityUnobscured {
                            Event::Window(EventWindow::Shown())
                        } else {
                            Event::Window(EventWindow::Hidden())
                        }
                    },
                    CreateNotify=> { println!("Display({:p}), CreateNotify({})", self, xevent._type); Event::Unknown },
                    DestroyNotify=> { println!("Display({:p}), DestroyNotify({})", self, xevent._type); Event::Unknown },
                    UnmapNotify=> { println!("Display({:p}), UnmapNotify({})", self, xevent._type); Event::Unknown },
                    MapNotify=> { println!("Display({:p}), MapNotify({})", self, xevent._type); Event::Unknown },
                    MapRequest=> { println!("Display({:p}), MapRequest({})", self, xevent._type); Event::Unknown },
                    ReparentNotify=> { println!("Display({:p}), ReparentNotify({})", self, xevent._type); Event::Unknown },

                    // Window position and/or size changed
                    ConfigureNotify=> { self.get_window_configuration_event() },

                    ConfigureRequest=> { println!("Display({:p}), ConfigureRequest({})", self, xevent._type); Event::Unknown },
                    GravityNotify=> { println!("Display({:p}), GravityNotify({})", self, xevent._type); Event::Unknown },

                    CirculateNotify=> { println!("Display({:p}), CirculateNotify({})", self, xevent._type); Event::Unknown },
                    CirculateRequest=> { println!("Display({:p}), CirculateRequest({})", self, xevent._type); Event::Unknown },
                    PropertyNotify=> { self.get_x11_window_states_event() },
                        
                    SelectionClear=> { println!("Display({:p}), SelectionClear({})", self, xevent._type); Event::Unknown },
                    SelectionRequest=> { println!("Display({:p}), SelectionRequest({})", self, xevent._type); Event::Unknown },
                    SelectionNotify=> { println!("Display({:p}), SelectionNotify({})", self, xevent._type); Event::Unknown },
                    ColormapNotify=> { println!("Display({:p}), ColormapNotify({})", self, xevent._type); Event::Unknown },
                    ClientMessage=> { println!("Display({:p}), ClientMessage({})", self, xevent._type); Event::Unknown },
                    MappingNotify=> { println!("Display({:p}), MappingNotify({})", self, xevent._type); Event::Unknown },
                    GenericEvent=> { println!("Display({:p}), GenericEvent({})", self, xevent._type); Event::Unknown },

    fn handle_keydown() {
        Event::Keyboard(EventKeyboard::KeyDown(xevent._xkey._keycode));
        }
        */

}
