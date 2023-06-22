use std::ffi::{c_ulong, c_ushort, c_char, CStr, c_long, c_int};


pub type XIM = c_ulong;
pub type XIC = c_ulong;

pub type XIMStyle = c_ulong;

pub const XIMPreeditNothing :c_long = 	0x0008;
pub const XIMStatusNothing:c_long = 0x0400;

pub const XNQueryInputStyle : &str = "queryInputStyle\0";
pub const XNInputStyle : &str =  "inputStyle\0";
pub const XNClientWindow : &str = "clientWindow\0";
pub const XNFocusWindow : &str = "focusWindow\0";

pub const XBufferOverflow : c_int =	-1;
pub const XLookupNone: c_int =		1;
pub const XLookupChars: c_int =		2;
pub const XLookupKeySym: c_int =		3;
pub const XLookupBoth: c_int =		4;

/// Keyboard description with name.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XIMStyles {
    pub count_styles : c_ushort,
    pub supported_styles : * mut XIMStyle,
}