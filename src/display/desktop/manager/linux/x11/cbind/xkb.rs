//! Structs used to fetch identities of Keycodes

use std::ffi::{c_ushort, c_uchar, c_char, c_uint};

use super::structs::{X11Display, XID, Atom};

/// Keysim is used to fetch char value.
pub type X11Keysim = c_uint;


const XKB_KEY_NAME_LENGTH : usize = 4;
const XKB_NUM_INDICATORS : usize = 32;
const XKB_NUM_KBD_GROUPS : usize =	4;
const XKB_NUM_VIRTUAL_MODS : usize = 16;

pub(crate) const XKB_ALL_COMPONENTS_MASK : u32 = (0x7f);
pub(crate) const XKB_USE_CORE_KBD : u32 =	0x0100;

/// Keyboard description with name.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct XkbDesc {
    dpy: *mut X11Display,
    flags: c_ushort,
    device_spec: c_ushort,
    min_key_code: c_uchar,
    pub max_key_code: c_uchar,
    ctrls: *mut XID,
    server: *mut XID,
    map: *mut XID,
    indicators: *mut XID,
    pub names: *mut XkbNames,
    compat: *mut XID,
    geom: *mut XID,
}
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate)  struct XkbNames {
    keycodes : Atom,
	geometry : Atom,
	symbols : Atom,
	types : Atom,
    compat : Atom,
    vmods : [Atom; XKB_NUM_VIRTUAL_MODS],
    indicators : [Atom; XKB_NUM_INDICATORS],
    groups : [Atom; XKB_NUM_KBD_GROUPS],
	pub keys : *mut XkbKeyName,
    // Rest not needed
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate)  struct XkbKeyName {
    pub name : [c_char; XKB_KEY_NAME_LENGTH],
}
