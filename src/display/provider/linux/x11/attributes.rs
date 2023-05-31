use std::{os::raw::{ c_int, c_ulong, c_long }, ptr::null_mut};

use super::event::{X11Handle, X11Display, XPointer, VisualID, Pixmap, Cursor};

use super::event::Colormap;

/// XWindowAttributes struct used to get x11 windows attributes with XGetWindowAttributes
#[repr(C)]
#[derive(Debug)]
pub(crate) struct XWindowAttributes {
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub depth: c_int,
    pub visual: *mut Visual,
    pub root: X11Handle,
    pub class: c_int,
    pub bit_gravity: c_int,
    pub win_gravity: c_int,
    pub backing_store: c_int,
    pub backing_planes: c_ulong,
    pub backing_pixel: c_ulong,
    pub save_under: bool,
    pub colormap: Colormap,
    pub map_installed: bool,
    pub map_state: c_int,
    pub all_event_masks: c_long,
    pub your_event_mask: c_long,
    pub do_not_propagate_mask: c_long,
    pub override_redirect: bool,
    pub screen: *mut Screen,
}

impl XWindowAttributes {
    /// Create a new empty XWindowAttributes set.
    pub(crate) fn empty() -> XWindowAttributes {
        XWindowAttributes { x: 0, y: 0, width: 0, height: 0, border_width: 0, 
            depth: 0, visual: null_mut(), root: 0, class: 0, bit_gravity: 0, win_gravity: 0, 
            backing_store:0, backing_planes: 0, backing_pixel: 0, save_under: false, colormap: 0, 
            map_installed: false, map_state: 0, all_event_masks: 0, your_event_mask: 0, 
            do_not_propagate_mask: 0, override_redirect: false, screen: null_mut() }
    }
}

/// XSetWindowAttributes is used to specify x11window attributes when creating them.
/// 
/// Reference(s)
/// <https://github.com/AltF02/x11-rs/blob/master/src/xlib.rs>
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub(crate) struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: c_ulong,
    pub bit_gravity: c_int,
    pub win_gravity: c_int,
    pub backing_store: c_int,
    pub backing_planes: c_ulong,
    pub backing_pixel: c_ulong,
    pub save_under: bool,
    pub event_mask: c_long,
    pub do_not_propagate_mask: c_long,
    pub override_redirect: bool,
    pub colormap: Colormap,
    pub cursor: Cursor,
}

/// Screen struct
/// Reference(s)
/// <https://github.com/AltF02/x11-rs/blob/master/src/xlib.rs>
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub(crate) struct Screen {
    pub ext_data: *mut XExtData,
    pub x11display: *mut X11Display,
    pub root: X11Handle,
    pub width: c_int,
    pub height: c_int,
    pub mwidth: c_int,
    pub mheight: c_int,
    pub ndepths: c_int,
    pub depths: *mut Depth,
    pub root_depth: c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: c_ulong,
    pub black_pixel: c_ulong,
    pub max_maps: c_int,
    pub min_maps: c_int,
    pub backing_store: c_int,
    pub save_unders: bool,
    pub root_input_mask: c_long,
}

pub(crate) type GC = *mut _XGC;

/// Opaque structure
/// Reference(s)
/// <https://github.com/AltF02/x11-rs/blob/master/src/xlib.rs>
pub(crate) enum _XGC {}

/// Depth struct
/// Reference(s)
/// <https://github.com/AltF02/x11-rs/blob/master/src/xlib.rs>
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub(crate) struct Depth {
    pub depth: c_int,
    pub nvisuals: c_int,
    pub visuals: *mut Visual,
}

/// Visual struct
/// Reference(s)
/// <https://github.com/AltF02/x11-rs/blob/master/src/xlib.rs>
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub(crate) struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: c_int,
    pub red_mask: c_ulong,
    pub green_mask: c_ulong,
    pub blue_mask: c_ulong,
    pub bits_per_rgb: c_int,
    pub map_entries: c_int,
}

/// External data struct
/// Reference(s)
/// <https://github.com/AltF02/x11-rs/blob/master/src/xlib.rs>
#[repr(C)]
pub(crate) struct XExtData {
    pub number: c_int,
    pub next: *mut XExtData,
    pub free_private: Option<unsafe extern "C" fn() -> c_int>,
    pub private_data: XPointer,
}