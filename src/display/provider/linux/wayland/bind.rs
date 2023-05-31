use std::os::raw::{c_char, c_int, c_void};

pub enum wl_proxy {}
pub enum wl_display {}
pub enum wl_event_queue {}

#[link(name = "wayland-client")]
extern {
    // display creation and destruction
    pub fn wl_display_connect_to_fd(fd : c_int) -> *mut wl_display;
    pub fn wl_display_connect(name : *const c_char) -> *mut wl_display;
    pub fn wl_display_disconnect(display : *mut wl_display) -> ();
    pub fn wl_display_get_fd(display : *mut wl_display) -> c_int;
}
