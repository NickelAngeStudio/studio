use std::ffi::CString;

use super::{bind::XInternAtom, event::Atom};

/// Macro that construct the X11Atoms struct. Since new atoms could be added,
/// this struct is generated by a macro for convenience.
macro_rules! x11_atoms_struct {
    ($atom:ident $(,$atoms:ident)*) => {
        /// Contains needed atoms for X window properties management.
        /// 
        /// Reference(s)
        /// <https://tronche.com/gui/x/xlib/window-information/properties-and-atoms.html>
        #[allow(non_snake_case)]
        #[derive(Debug)]
        pub struct X11Atoms {
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
        }

        impl X11Atoms{
            /// Fetch atoms value with display
            pub fn new(display : *mut u64) -> X11Atoms {
                #[allow(temporary_cstring_as_ptr)]
                unsafe {        
                    let x11atom = X11Atoms { _NET_WM_STATE_REMOVE : 0, _NET_WM_STATE_ADD : 1, _NET_WM_STATE_TOGGLE : 2,
                    $atom :  XInternAtom(display, CString::new(stringify!($atom)).unwrap().as_ptr(), true),
                    $($atoms : XInternAtom(display, CString::new(stringify!($atoms)).unwrap().as_ptr(), true),)*
                    xa_atom : 4 };

                    // Make sure that all Atoms have value > 0.
                    assert_ne!(x11atom.$atom, 0, "Atom [{}] value must NOT be 0.", stringify!($atom));

                    // Return X11 Atoms
                    x11atom
                }
            }
        }

        

    }


}



// Construct X11Atoms with needed atoms 
x11_atoms_struct!(_NET_WM_STATE, _NET_WM_STATE_MAXIMIZED_VERT, _NET_WM_STATE_MAXIMIZED_HORZ, _NET_WM_STATE_HIDDEN, _NET_WM_STATE_FULLSCREEN,
    _NET_WM_WINDOW_TYPE, _NET_WM_WINDOW_TYPE_NORMAL, _NET_WM_ALLOWED_ACTIONS, _NET_WM_ACTION_FULLSCREEN,
    _NET_WM_ACTION_MINIMIZE, _NET_WM_ACTION_CHANGE_DESKTOP, _NET_WM_ACTION_CLOSE, _NET_WM_ACTION_ABOVE, _NET_WM_ACTION_BELOW
);