// Generated with "script/rustify_x11_event.sh"
// NOTE: All struct and union members name start with "_" to prevent conflict with Rust reserved words.

use std::os::raw::{ c_int, c_long, c_uint, c_ulong, c_char, c_uchar, c_short, c_void };
// Types definition (ref : https://docs.rs/x11/latest/x11)
pub type Time = c_ulong;
pub type XID = c_ulong;
pub type Atom = XID;
pub type Colormap = XID;
pub type Drawable = XID;

/// Type used for x11display server x11window handle.
pub type X11Handle = c_ulong;

/// Type used for x11display server connection pointer. 
pub type X11Display = c_ulong;

/// Type used for pix map
pub type Pixmap = c_ulong;

/// XPointer type
pub type XPointer = *mut c_char;

/// Visual ID type
pub type VisualID = c_ulong;

/// Cursor type
pub type Cursor = c_ulong;

/// Union 'data' of XClientMessageEvent struct.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XClientMessageEvent_data {
pub _b : [c_char; 20],
pub _s : [c_short; 10],
pub _l : [c_long; 5]
}
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XKeyEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _window:X11Handle,
	pub _root:X11Handle,
	pub _subwindow:X11Handle,
	pub _time:Time,
	pub _x:c_int,
	pub _y:c_int,
	pub _x_root:c_int,
	pub _y_root:c_int,
	pub _state:c_uint,
	pub _keycode:c_uint,
	pub _same_screen:bool,
}

pub type XKeyPressedEvent = XKeyEvent;
pub type XKeyReleasedEvent = XKeyEvent;
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XButtonEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _window:X11Handle,
	pub _root:X11Handle,
	pub _subwindow:X11Handle,
	pub _time:Time,
	pub _x:c_int,
	pub _y:c_int,
	pub _x_root:c_int,
	pub _y_root:c_int,
	pub _state:c_uint,
	pub _button:c_uint,
	pub _same_screen:bool,
}

pub type XButtonPressedEvent = XButtonEvent;
pub type XButtonReleasedEvent = XButtonEvent;
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XMotionEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _window:X11Handle,
	pub _root:X11Handle,
	pub _subwindow:X11Handle,
	pub _time:Time,
	pub _x:c_int,
	pub _y:c_int,
	pub _x_root:c_int,
	pub _y_root:c_int,
	pub _state:c_uint,
	pub _is_hint:c_int,
	pub _same_screen:bool,
}

pub type XPointerMovedEvent = XMotionEvent;
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XCrossingEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _window:X11Handle,
	pub _root:X11Handle,
	pub _subwindow:X11Handle,
	pub _time:Time,
	pub _x:c_int,
	pub _y:c_int,
	pub _x_root:c_int,
	pub _y_root:c_int,
	pub _mode:c_int,
	pub _detail:c_int,
	pub _same_screen:bool,
	pub _focus:bool,
	pub _state:c_uint,
}

pub type XEnterWindowEvent = XCrossingEvent;
pub type XLeaveWindowEvent = XCrossingEvent;
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XFocusChangeEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _window:X11Handle,
	pub _mode:c_int,
	pub _detail:c_int,
}

pub type XFocusInEvent = XFocusChangeEvent;
pub type XFocusOutEvent = XFocusChangeEvent;
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XKeymapEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _window:X11Handle,
	pub _key_vector:[c_char; 32],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XExposeEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _window:X11Handle,
	pub _x:c_int,
	pub _y:c_int,
	pub _width:c_int,
	pub _height:c_int,
	pub _count:c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XGraphicsExposeEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _drawable:Drawable,
	pub _x:c_int,
	pub _y:c_int,
	pub _width:c_int,
	pub _height:c_int,
	pub _count:c_int,
	pub _major_code:c_int,
	pub _minor_code:c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XNoExposeEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _drawable:Drawable,
	pub _major_code:c_int,
	pub _minor_code:c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XVisibilityEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _window:X11Handle,
	pub _state:c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XCreateWindowEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _parent:X11Handle,
	pub _window:X11Handle,
	pub _x:c_int,
	pub _y:c_int,
	pub _width:c_int,
	pub _height:c_int,
	pub _border_width:c_int,
	pub _override_redirect:bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XDestroyWindowEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _event:X11Handle,
	pub _window:X11Handle,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XUnmapEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _event:X11Handle,
	pub _window:X11Handle,
	pub _from_configure:bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XMapEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _event:X11Handle,
	pub _window:X11Handle,
	pub _override_redirect:bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XMapRequestEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _parent:X11Handle,
	pub _window:X11Handle,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XReparentEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _event:X11Handle,
	pub _window:X11Handle,
	pub _parent:X11Handle,
	pub _x:c_int,
	pub _y:c_int,
	pub _override_redirect:bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XConfigureEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _event:X11Handle,
	pub _window:X11Handle,
	pub _x:c_int,
	pub _y:c_int,
	pub _width:c_int,
	pub _height:c_int,
	pub _border_width:c_int,
	pub _above:X11Handle,
	pub _override_redirect:bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XGravityEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _event:X11Handle,
	pub _window:X11Handle,
	pub _x:c_int,
	pub _y:c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XResizeRequestEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _window:X11Handle,
	pub _width:c_int,
	pub _height:c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XConfigureRequestEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _parent:X11Handle,
	pub _window:X11Handle,
	pub _x:c_int,
	pub _y:c_int,
	pub _width:c_int,
	pub _height:c_int,
	pub _border_width:c_int,
	pub _above:X11Handle,
	pub _detail:c_int,
	pub _value_mask:c_ulong,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XCirculateEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _event:X11Handle,
	pub _window:X11Handle,
	pub _place:c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XCirculateRequestEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _parent:X11Handle,
	pub _window:X11Handle,
	pub _place:c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XPropertyEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _window:X11Handle,
	pub _atom:Atom,
	pub _time:Time,
	pub _state:c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XSelectionClearEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _window:X11Handle,
	pub _selection:Atom,
	pub _time:Time,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XSelectionRequestEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _owner:X11Handle,
	pub _requestor:X11Handle,
	pub _selection:Atom,
	pub _target:Atom,
	pub _property:Atom,
	pub _time:Time,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XSelectionEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _requestor:X11Handle,
	pub _selection:Atom,
	pub _target:Atom,
	pub _property:Atom,
	pub _time:Time,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XColormapEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _window:X11Handle,
	pub _new:bool,
	pub _state:c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XClientMessageEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _window:X11Handle,
	pub _message_type:Atom,
	pub _format:c_int,
	pub _data:XClientMessageEvent_data,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XMappingEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _window:X11Handle,
	pub _request:c_int,
	pub _first_keycode:c_int,
	pub _count:c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XErrorEvent {
	pub _type:c_int,
	pub _display:*mut X11Display,
	pub _serial:c_ulong,
	pub _error_code:c_uchar,
	pub _request_code:c_uchar,
	pub _minor_code:c_uchar,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XAnyEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _Display:X11Display,
	pub _the:X11Display,
	pub _event:X11Display,
	pub _window:X11Handle,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XGenericEvent {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _extension:c_int,
	pub _evtype:c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XGenericEventCookie {
	pub _type:c_int,
	pub _serial:c_ulong,
	pub _send_event:bool,
	pub _display:*mut X11Display,
	pub _extension:c_int,
	pub _evtype:c_int,
	pub _cookie:c_uint,
	pub _data:*mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union XEvent {
	pub _type:c_int,
	pub _xany:XAnyEvent,
	pub _xkey:XKeyEvent,
	pub _xbutton:XButtonEvent,
	pub _xmotion:XMotionEvent,
	pub _xcrossing:XCrossingEvent,
	pub _xfocus:XFocusChangeEvent,
	pub _xexpose:XExposeEvent,
	pub _xgraphicsexpose:XGraphicsExposeEvent,
	pub _xnoexpose:XNoExposeEvent,
	pub _xvisibility:XVisibilityEvent,
	pub _xcreatewindow:X11Handle,
	pub _xdestroywindow:X11Handle,
	pub _xunmap:XUnmapEvent,
	pub _xmap:XMapEvent,
	pub _xmaprequest:XMapRequestEvent,
	pub _xreparent:XReparentEvent,
	pub _xconfigure:XConfigureEvent,
	pub _xgravity:XGravityEvent,
	pub _xresizerequest:XResizeRequestEvent,
	pub _xconfigurerequest:XConfigureRequestEvent,
	pub _xcirculate:XCirculateEvent,
	pub _xcirculaterequest:XCirculateRequestEvent,
	pub _xproperty:XPropertyEvent,
	pub _xselectionclear:XSelectionClearEvent,
	pub _xselectionrequest:XSelectionRequestEvent,
	pub _xselection:XSelectionEvent,
	pub _xcolormap:XColormapEvent,
	pub _xclient:XClientMessageEvent,
	pub _xmapping:XMappingEvent,
	pub _xerror:XErrorEvent,
	pub _xkeymap:XKeymapEvent,
	pub _xgeneric:XGenericEvent,
	pub _xcookie:XGenericEventCookie,
	pub _pad:[c_long; 24],
}
