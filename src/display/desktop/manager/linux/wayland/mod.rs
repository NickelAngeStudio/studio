use cfg_boost::target_cfg;

use crate::{display::desktop::{manager::WindowManager, manager::{WindowProvider}, Window, property::{KeyboardMode}, window::WindowShowOption}, error::StudioError};

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
}

impl WindowManager for WaylandWindowManager {
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

    target_cfg! {
        !immediate:ft => {  // Retained mode
            fn show(&mut self, _option : WindowShowOption, _parent : Option<&Window>) {
                todo!()
            }

            #[inline(always)]
            fn poll_event(&mut self) -> crate::display::desktop::event::Event  {
                todo!()
            }
        },
        immediate:ft => {   // Immediate mode
            fn show(&mut self) {
                todo!()
            }

            #[inline(always)]
            fn poll_event(&mut self) -> &crate::display::desktop::event::Event  {
                todo!()
            }
        }
    }

    

    fn close(&mut self) {
        todo!()
    }

    fn restore(&mut self) {
        todo!()
    }


    fn push_event(&self, _event: crate::display::desktop::event::Event) {
        todo!()
    }
   

    fn set_title(&mut self, _title : &String){
        todo!()
    }

    fn set_position(&mut self, _option : crate::display::desktop::property::WindowPositionOption){
        todo!()
    }

    fn set_size(&mut self, _size : &(u32,u32)){
        todo!()
    }

    fn show_decoration(&mut self){
        todo!()
    }

    fn hide_decoration(&mut self){
        todo!()
    }

    fn minimize(&mut self){
        todo!()
    }

    fn maximize(&mut self){
        todo!()
    }

    fn set_fullscreen(&mut self, _fsmode : crate::display::desktop::property::FullScreenMode){
        todo!()
    }

    fn set_keyboard_mode(&mut self, _mode : KeyboardMode){
        todo!()
    }

    #[inline(always)]
    fn set_keyboard_auto_repeat(&mut self, auto_repeat : bool){
        todo!()
    }

    fn set_pointer_mode(&mut self, _mode : &crate::display::desktop::property::PointerMode){
        todo!()
    }

    fn set_pointer_position(&mut self, _position : (i32, i32)){
        todo!()
    }

    fn show_pointer(&mut self){
        todo!()
    }

    fn hide_pointer(&mut self){
        todo!()
    }

    fn confine_pointer(&mut self){
        todo!()
    }

    fn release_pointer(&mut self){
        todo!()
    }

    fn is_key_shift_down(_state : u32) -> bool {
        todo!()
    }

    fn is_key_ctrl_down(_state : u32) -> bool {
        todo!()
    }

    fn is_key_alt_down(_state : u32) -> bool {
        todo!()
    }

    fn is_key_meta_down(_state : u32) -> bool {
        todo!()
    }

    fn is_key_command_down(_state : u32) -> bool {
        todo!()
    }

    fn is_key_hyper_down(_state : u32) -> bool {
        todo!()
    }

    fn is_capslock_on(_state : u32) -> bool {
        todo!()
    }

    fn is_numlock_on(_state : u32) -> bool {
        todo!()
    }

    fn refresh(&mut self) {
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

