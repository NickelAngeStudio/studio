use crate::{display::desktop::{manager::WindowManager, provider::WindowProvider}, error::StudioError};

/// Static cache to know if wayland is supported
#[doc(hidden)]
pub static WaylandSupported : Option<bool> = None;

pub(crate) struct WaylandWindowManager{

}

impl WaylandWindowManager {
    pub fn is_supported() -> bool { 
        match WaylandSupported {
            Some(supported) => supported,
            None => {
                unsafe {
                    WaylandSupported = Some(false);
                }
                false
            },
        }
        
    }
}

impl WindowManager for WaylandWindowManager {
    fn new() -> Result<Self, StudioError> where Self : Sized {
        todo!()
    }

    #[inline(always)]
    fn get_window_provider(&self) -> WindowProvider {
        WindowProvider::Wayland
    }

    #[inline(always)]
    fn poll_event(&mut self) -> crate::display::desktop::event::Event  {
        todo!()
    }

    #[inline(always)]
    fn show(&mut self, property : &crate::display::desktop::property::WindowProperty) {
        todo!()
    }

    #[inline(always)]
    fn restore(&mut self) {
        todo!()
    }

    #[inline(always)]
    fn close(&mut self) {
        todo!()
    }

    #[inline(always)]
    fn hide(&mut self) {
        todo!()
    }

    #[inline(always)]
    fn set_title(&mut self, title : &String) -> bool {
        todo!()
    }

    #[inline(always)]
    fn set_position(&mut self, position : (i32,i32)) -> bool {
        todo!()
    }

    #[inline(always)]
    fn set_size(&mut self, size : &(u32,u32)) -> bool {
        todo!()
    }

    #[inline(always)]
    fn show_decoration(&mut self) -> bool {
        todo!()
    }

    #[inline(always)]
    fn hide_decoration(&mut self) -> bool {
        todo!()
    }

    #[inline(always)]
    fn minimize(&mut self) -> bool {
        todo!()
    }

    #[inline(always)]
    fn maximize(&mut self) -> bool {
        todo!()
    }

    #[inline(always)]
    fn enable_autorepeat(&mut self) -> bool {
        todo!()
    }

    #[inline(always)]
    fn disable_autorepeat(&mut self) -> bool {
        todo!()
    }

    #[inline(always)]
    fn set_pointer_position(&mut self, position : &(i32, i32)) -> bool {
        todo!()
    }

    #[inline(always)]
    fn show_pointer(&mut self) -> bool {
        todo!()
    }

    #[inline(always)]
    fn hide_pointer(&mut self) -> bool {
        todo!()
    }

    #[inline(always)]
    fn confine_pointer(&mut self) -> bool {
        todo!()
    }

    #[inline(always)]
    fn release_pointer(&mut self) -> bool {
        todo!()
    }

    #[inline(always)]
    fn get_window_handle(&self) -> Option<*const usize> {
        todo!()
    }  

    #[inline(always)]
    fn get_display_handle(&self) -> Option<*const usize> {
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

