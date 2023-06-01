use std::{os::raw::{ c_char, c_ulong }, ffi::{CString}, any::Any};
use debug_print::debug_println;

use crate::{kleio::display::{KWindow, event::{window, KEvent}}, wayland_or_x11, error::{StudioError, KWindowError}};

use super::{x11::{event::{XEvent, Atom}, bind::{XCloseDisplay, XInternAtom, XDefaultRootWindow}}};


/// Enumeration of linux display server provider.
/// 
/// Linux can support more than 1 display server so it is important to enumerate
/// supported display server and be ready for future addition.
#[cfg(any(doc, all(not(target_family = "wasm"), target_os = "linux")))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "linux"))))]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KLinuxDisplayServerProvider {

    /// [Wayland](https://en.wikipedia.org/wiki/Wayland_(protocol)) display server.
    Wayland,

    /// [X Window System](https://en.wikipedia.org/wiki/X_Window_System) display server.
    X11,
}


/// Contains elements relatives to X11 and Wayland display server.
pub struct KLinuxDisplayServer {

    /// Used to determine which provider is used
    pub provider : KLinuxDisplayServerProvider,

    /// X11 only properties
    pub x11_property : KLinuxDisplayServerX11Property,

    /// Display connection pointer
    pub display : *mut Display,

    /// Window handle pointer
    pub window : *mut Window,


}


impl KLinuxDisplayServer {
    /// Create a new KLinuxDisplayServer according to provider.
    pub fn new(width:u32, height:u32, provider : KLinuxDisplayServerProvider) -> Result<KLinuxDisplayServer, StudioError> {
        match provider {
            KLinuxDisplayServerProvider::Default => {

                match KLinuxDisplayServer::new(width, height, KLinuxDisplayServerProvider::Wayland) {
                    Ok(klds) => Ok(klds),
                    Err(_) => KLinuxDisplayServer::new(width, height, KLinuxDisplayServerProvider::X11),
                }
            },
            KLinuxDisplayServerProvider::Wayland => {
                if KWindow::wayland_supported() {
                    let provider = KLinuxDisplayServerProvider::Wayland;
                    let dis_win = KWindow::create_wayland_window(width, height);

                    Ok(KLinuxDisplayServer{ provider, x11_property : KLinuxDisplayServerX11Property::empty(), display : dis_win.0, window : dis_win.1 })

                } else {
                    // No wayland support.
                    Err(StudioError::KWindow(KWindowError::NotSupported))
                }
            },
            KLinuxDisplayServerProvider::X11 => {
                if KWindow::x11_supported() {
                    let provider = KLinuxDisplayServerProvider::X11;

                    let prop_dis = KWindow::create_x11_display_connection();
                    let window = KWindow::create_x11_window(prop_dis.1, KWindow::get_x11_default_root_window(prop_dis.1), &prop_dis.0, (0,0), (width, height), false);
                    Ok(KLinuxDisplayServer{ provider, x11_property : prop_dis.0, display : prop_dis.1, window : window })
                } else {
                    // No x11 support.
                    Err(StudioError::KWindow(KWindowError::NotSupported))
                }
            },
        }

    }

}

impl Drop for KLinuxDisplayServer {
    fn drop(&mut self) {
        wayland_or_x11!{self.provider, {
            todo!()
        } , {
            unsafe {
                // Close display server connection.
                XCloseDisplay(self.display);
            }
        }}
    }
}


/// Macro that construct the KLinuxDisplayServerX11Property struct
macro_rules! x11_server_property {
    ($atom:ident $(,$atoms:ident)*) => {
        #[allow(non_snake_case)]
        pub struct KLinuxDisplayServerX11Property {
            /// Used to fetch X11 events
            pub(crate) x_event : XEvent,    

            /// C-compatible string for window title
            pub(crate) wm_title : CString,

            // Remove/unset property
            pub(crate) _NET_WM_STATE_REMOVE : Atom,

            // Add/set property
            pub(crate) _NET_WM_STATE_ADD : Atom,

            // Toggle property
            pub(crate) _NET_WM_STATE_TOGGLE : Atom,

            // List of atoms to use (Filled with atoms name use when calling x11_server_property! macro)
            pub(crate) $atom : Atom,
            $(pub(crate) $atoms : Atom,)*

            /// Used to query atom type
            pub(crate) xa_atom : Atom,

            /// Flag used to make sure XHideCursor was called prior to XShowCursor to prevent crash
            pub(crate) x_hide_cursor_flag : bool,

            /// Position and size for restoring window.
            pub(crate) restoration_position_size : ((i32,i32),(u32,u32)),
        }

        impl KLinuxDisplayServerX11Property{
            /// Fetch atoms value with display
            pub fn new(display : *mut u64) -> KLinuxDisplayServerX11Property {
                #[allow(temporary_cstring_as_ptr)]
                unsafe {        
                    let x11_prop = KLinuxDisplayServerX11Property { x_event : XEvent { _type: 0}, wm_title : CString::new("").unwrap(),
                    _NET_WM_STATE_REMOVE : 0, _NET_WM_STATE_ADD : 1, _NET_WM_STATE_TOGGLE : 2,
                    $atom :  XInternAtom(display, CString::new(stringify!($atom)).unwrap().as_ptr(), true),
                    $($atoms : XInternAtom(display, CString::new(stringify!($atoms)).unwrap().as_ptr(), true),)*
                    xa_atom : 4, x_hide_cursor_flag : false, restoration_position_size : ((0,0),(0,0)) };

                    // Make sure that all Atoms have value > 0.
                    assert_ne!(x11_prop.$atom, 0, "Atom [{}] value must NOT be 0.", stringify!($atom));

                    // Return propertis
                    x11_prop
                }
            }
        
            /// Empty X11 Atoms
            pub fn empty() -> KLinuxDisplayServerX11Property {
                KLinuxDisplayServerX11Property { x_event : XEvent { _type: 0}, wm_title : CString::new("").unwrap(),
                    _NET_WM_STATE_REMOVE : 0, _NET_WM_STATE_ADD : 1, _NET_WM_STATE_TOGGLE : 2,
                    $atom :  0,
                    $($atoms : 0,)*
                    xa_atom : 4, x_hide_cursor_flag : false,
                    restoration_position_size : ((0,0),(0,0)) }
            }
        }

        

    }


}



// Construct KLinuxDisplayServerX11Property with needed atoms 
x11_server_property!(_NET_WM_STATE, _NET_WM_STATE_MAXIMIZED_VERT, _NET_WM_STATE_MAXIMIZED_HORZ, _NET_WM_STATE_HIDDEN, _NET_WM_STATE_FULLSCREEN,
    _NET_WM_WINDOW_TYPE, _NET_WM_WINDOW_TYPE_NORMAL, _NET_WM_ALLOWED_ACTIONS, _NET_WM_ACTION_FULLSCREEN,
    _NET_WM_ACTION_MINIMIZE, _NET_WM_ACTION_CHANGE_DESKTOP, _NET_WM_ACTION_CLOSE, _NET_WM_ACTION_ABOVE, _NET_WM_ACTION_BELOW
);


impl core::fmt::Debug for KLinuxDisplayServerX11Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KLinuxDisplayServerX11Property").field("wm_title", &self.wm_title).field("_NET_WM_STATE_REMOVE", &self._NET_WM_STATE_REMOVE).field("_NET_WM_STATE_ADD", &self._NET_WM_STATE_ADD).field("_NET_WM_STATE_TOGGLE", &self._NET_WM_STATE_TOGGLE).field("_NET_WM_STATE", &self._NET_WM_STATE).field("_NET_WM_STATE_MAXIMIZED_VERT", &self._NET_WM_STATE_MAXIMIZED_VERT).field("_NET_WM_STATE_MAXIMIZED_HORZ", &self._NET_WM_STATE_MAXIMIZED_HORZ).field("_NET_WM_STATE_HIDDEN", &self._NET_WM_STATE_HIDDEN).field("_NET_WM_STATE_FULLSCREEN", &self._NET_WM_STATE_FULLSCREEN).field("_NET_WM_WINDOW_TYPE", &self._NET_WM_WINDOW_TYPE).field("_NET_WM_WINDOW_TYPE_NORMAL", &self._NET_WM_WINDOW_TYPE_NORMAL).field("_NET_WM_ALLOWED_ACTIONS", &self._NET_WM_ALLOWED_ACTIONS).field("_NET_WM_ACTION_FULLSCREEN", &self._NET_WM_ACTION_FULLSCREEN).field("_NET_WM_ACTION_MINIMIZE", &self._NET_WM_ACTION_MINIMIZE).field("_NET_WM_ACTION_CHANGE_DESKTOP", &self._NET_WM_ACTION_CHANGE_DESKTOP).field("_NET_WM_ACTION_CLOSE", &self._NET_WM_ACTION_CLOSE).field("_NET_WM_ACTION_ABOVE", &self._NET_WM_ACTION_ABOVE).field("_NET_WM_ACTION_BELOW", &self._NET_WM_ACTION_BELOW).field("xa_atom", &self.xa_atom).field("x_hide_cursor_flag", &self.x_hide_cursor_flag).finish()
    }
}
