use crate::{display::desktop::{manager::WindowManager, manager::WindowProvider, Window, event::keyboard::Key, property::{KeyboardMode, WindowEventWaitMode}}, error::StudioError};

/// Static cache to know if wayland is supported
#[doc(hidden)]
pub static mut WAYLAND_SUPPORTED : Option<bool> = None;

pub(crate) struct WaylandWindowManager{

}

impl WaylandWindowManager {
    pub fn is_supported() -> bool { 
        match unsafe { WAYLAND_SUPPORTED } {
            Some(supported) => supported,
            None => {
                unsafe {
                    WAYLAND_SUPPORTED = Some(false);
                }
                false
            },
        }
        
    }

    pub fn get_char(key : &Key) -> Option<char> {
        todo!()
    }
}

impl<'window> WindowManager<'window> for WaylandWindowManager {
    fn new() -> Result<Self, StudioError> where Self : Sized {
        todo!()
    }

    #[inline(always)]
    fn get_window_provider(&self) -> WindowProvider {
        WindowProvider::Wayland
    }

    fn get_properties(&self) -> &crate::display::desktop::property::WindowProperty {
        todo!()
    }

    fn poll_event(&mut self) -> &crate::display::desktop::event::Event {
        todo!()
    }

    fn recreate(&mut self) {
        todo!()
    }

    fn show(&mut self) {
        todo!()
    }

    fn close(&mut self) {
        todo!()
    }

    fn hide(&mut self) {
        todo!()
    }

    fn restore(&mut self) {
        todo!()
    }

    fn get_window_handle(&self) -> Option<*const usize> {
        todo!()
    }

    fn push_event(&self, _event: crate::display::desktop::event::Event) {
        todo!()
    }

    #[cfg(any(doc,target_os = "linux"))]
#[cfg_attr(docsrs,doc(cfg(target_os = "linux")))]
#[doc = " Get the OS Window manager display handle."]
fn get_display_handle(&self) ->  *const usize {
        todo!()
    }

   

    fn remove_parent(&mut self) -> bool {
        todo!()
    }

    fn set_title(&mut self, _title : &String) -> bool {
        todo!()
    }

    fn set_position(&mut self, _option : crate::display::desktop::property::WindowPositionOption) -> bool {
        todo!()
    }

    fn set_size(&mut self, _size : &(u32,u32)) -> bool {
        todo!()
    }

    fn show_decoration(&mut self) -> bool {
        todo!()
    }

    fn hide_decoration(&mut self) -> bool {
        todo!()
    }

    fn minimize(&mut self) -> bool {
        todo!()
    }

    fn maximize(&mut self) -> bool {
        todo!()
    }

    #[inline(always)]
    fn set_event_wait_mode(&mut self, mode : WindowEventWaitMode) -> bool {
        todo!()
    }

    fn set_fullscreen(&mut self, _fsmode : crate::display::desktop::property::FullScreenMode) -> bool {
        todo!()
    }

    fn set_keyboard_mode(&mut self, _mode : KeyboardMode) -> bool {
        todo!()
    }

    fn set_pointer_mode(&mut self, _mode : &crate::display::desktop::property::PointerMode) -> bool {
        todo!()
    }

    fn set_pointer_position(&mut self, _position : (i32, i32)) -> bool {
        todo!()
    }

    fn show_pointer(&mut self) -> bool {
        todo!()
    }

    fn hide_pointer(&mut self) -> bool {
        todo!()
    }

    fn confine_pointer(&mut self) -> bool {
        todo!()
    }

    fn release_pointer(&mut self) -> bool {
        todo!()
    }

    fn set_parent<'manager: 'window>(&mut self, _parent : &'manager Window<'manager>, _option : crate::display::desktop::property::SubWindowOption) -> bool {
        todo!()
    }

    fn is_key_shift_down(state : u32) -> bool {
        todo!()
    }

    fn is_key_ctrl_down(state : u32) -> bool {
        todo!()
    }

    fn is_key_alt_down(state : u32) -> bool {
        todo!()
    }

    fn is_key_meta_down(state : u32) -> bool {
        todo!()
    }

    fn is_key_command_down(state : u32) -> bool {
        todo!()
    }

    fn is_key_hyper_down(state : u32) -> bool {
        todo!()
    }

    fn is_capslock_on(state : u32) -> bool {
        todo!()
    }

    fn is_numlock_on(state : u32) -> bool {
        todo!()
    }


}

/*
/// Get if Wayland is supported.
pub(crate) fn wayland_supported() -> bool {
    unsafe {
        // Try to call C function with error handling.
        let result = catch_unwind(|| {
            wl_display_connect(std::ptr::null())
        }); 
        match result {
            Ok(display) => {
                if display == std::ptr::null_mut() {
                    false
                } else {
                    // Disconnect display before returning true
                    wl_display_disconnect(display);

                    true
                }

            },
            // C function crashed. Wayland not supported.
            Err(_) => false,
        }
    }
}
*/

