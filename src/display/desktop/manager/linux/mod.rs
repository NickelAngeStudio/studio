//! Linux implementations of [WindowManager].

use cfg_boost::target_cfg;
use tests_bin::unit_tests;

use crate::{display::{ desktop::{ manager::WindowManager, window::{  Window, WindowShowOption}, event::{Event}, property::{WindowProperty, WindowPositionOption, KeyboardMode}}, DisplayError}, error::StudioError};
use self::{wayland::{WaylandWindowManager, WAYLAND_SUPPORTED}, x11::X11WindowManager};
use super::{WindowProvider};

/// Wayland DisplayManager
pub mod wayland;

/// X11 DisplayManager
pub mod x11;


/// Enumeration of implemented [WindowManager]
pub(crate) enum ImplementedLinuxWindowManager{
    Wayland(WaylandWindowManager),
    X11(X11WindowManager)
}

/// Macro that redirect function to correct window manager. 
macro_rules! wmfn {
    ($self : ident, $funct : ident ( $($param : tt)* )) => {
        match &$self.wm{
            ImplementedLinuxWindowManager::Wayland(wm) => wm.$funct($($param)*),
            ImplementedLinuxWindowManager::X11(wm) => wm.$funct($($param)*),
        }
    };

    (mut $self : ident, $funct : ident ( $($param : tt)* )) => {
        match &mut $self.wm{
            ImplementedLinuxWindowManager::Wayland(wm) => wm.$funct($($param)*),
            ImplementedLinuxWindowManager::X11(wm) => wm.$funct($($param)*),
        }
    };
}

/// Macro that redirect function to correct window manager for static call. 
macro_rules! static_wmfn {
    ($funct : ident ( $($param : tt)* )) => {
        match unsafe { WAYLAND_SUPPORTED } {
            Some(supported) => if supported {
                WaylandWindowManager::$funct($($param)*)
            } else {
                X11WindowManager::$funct($($param)*)
            },
            None => X11WindowManager::$funct($($param)*),
        } 
    };
}


/// Linux [WindowManager] that act as a dispatcher between Wayland and X11.
#[unit_tests("display/desktop/manager/linux.rs")]
pub struct LinuxWindowManager {
    wm : ImplementedLinuxWindowManager,
}

impl LinuxWindowManager {
    /// Get the X11 WindowManager
    pub(crate) fn get_x11_wm(&self) -> &X11WindowManager{
        match &self.wm{
            ImplementedLinuxWindowManager::Wayland(_) => panic!("Wrong Linux Window Manager for this function!"),
            ImplementedLinuxWindowManager::X11(x11wm) => &x11wm,
        }
    }

    /* TODO: Unlock when implementing WaylandWindowManager
    /// Get the Wayland Window Manager
    pub(crate) fn get_wayland_wm(&self) -> &WaylandWindowManager{
        match &self.wm{
            ImplementedLinuxWindowManager::Wayland(wlWm) => &wlWm,
            ImplementedLinuxWindowManager::X11(x11wm) => panic!("Wrong Linux Window Manager for this function!"),
        }
    }
    */
}

impl WindowManager for LinuxWindowManager {
    fn new() -> Result<Self, StudioError> where Self : Sized {
        
        if wayland::WaylandWindowManager::is_supported() {
            Ok(LinuxWindowManager{ 
                wm : ImplementedLinuxWindowManager::Wayland(wayland::WaylandWindowManager::new().unwrap())
            })
        } else if x11::X11WindowManager::is_supported() {
            Ok(LinuxWindowManager{ 
                wm : ImplementedLinuxWindowManager::X11(x11::X11WindowManager::new().unwrap())
            })
        } else {    // No supported display server available
            Err(StudioError::Display(DisplayError::NoDisplayServer))
        }

    }

    #[inline(always)]
    fn get_window_provider(&self) -> WindowProvider {
        wmfn!(self, get_window_provider())
    }

    target_cfg! {
        !immediate:ft => {  // Retained mode
            #[inline(always)]
            fn show(&mut self, option : WindowShowOption, parent : Option<&Window>) {
                wmfn!(mut self, show(option, parent))
            }

            #[inline(always)]
            fn poll_event(&mut self) -> Event  {
                wmfn!(mut self, poll_event())
            }
        },
        immediate:ft => {   // Immediate mode
            #[inline(always)]
            fn poll_event(&mut self) -> &Event  {
                wmfn!(mut self, poll_event())
            }

            #[inline(always)]
            fn show(&mut self) {
                wmfn!(mut self, show())
            }
        }
    }

    fn push_event(&self, event: Event){
        wmfn!(self, push_event(event))
    }

    

    #[inline(always)]
    fn close(&mut self) {
         wmfn!(mut self, close());
    }

    fn refresh(&mut self) {
        wmfn!(mut self, refresh());
    }

    #[inline(always)]
    fn set_title(&mut self, title : &String){
         wmfn!(mut self, set_title(title))
    }

    #[inline(always)]
    fn set_position(&mut self, option : WindowPositionOption){
         wmfn!(mut self, set_position(option))
    }

    #[inline(always)]
    fn set_size(&mut self, size : &(u32,u32)){
         wmfn!(mut self, set_size(size))
    }

    #[inline(always)]
    fn show_decoration(&mut self){
         wmfn!(mut self, show_decoration())
    }

    #[inline(always)]
    fn hide_decoration(&mut self){
         wmfn!(mut self, hide_decoration())
    }

    #[inline(always)]
    fn minimize(&mut self){
         wmfn!(mut self, minimize())
    }

    #[inline(always)]
    fn maximize(&mut self){
         wmfn!(mut self, maximize())
    }

    #[inline(always)]
    fn set_keyboard_mode(&mut self, mode : KeyboardMode){
         wmfn!(mut self, set_keyboard_mode(mode))
    }

    #[inline(always)]
    fn set_keyboard_auto_repeat(&mut self, auto_repeat : bool){
        wmfn!(mut self, set_keyboard_auto_repeat(auto_repeat))
    }

    #[inline(always)]
    fn set_pointer_position(&mut self, position : (i32, i32)){
         wmfn!(mut self, set_pointer_position(position))
    }

    #[inline(always)]
    fn show_pointer(&mut self){
         wmfn!(mut self, show_pointer())
    }

    #[inline(always)]
    fn hide_pointer(&mut self){
         wmfn!(mut self, hide_pointer())
    }

    #[inline(always)]
    fn confine_pointer(&mut self){
         wmfn!(mut self, confine_pointer())
    }

    #[inline(always)]
    fn release_pointer(&mut self){
         wmfn!(mut self, release_pointer())
    }

    fn get_properties(&self) -> &WindowProperty {
        wmfn!(self, get_properties())
    }

    fn restore(&mut self) {
        wmfn!(mut self, restore());
    }

    fn set_fullscreen(&mut self, fsmode : crate::display::desktop::property::FullScreenMode){
        wmfn!(mut self, set_fullscreen(fsmode))
    }

    fn set_pointer_mode(&mut self, mode : &crate::display::desktop::property::PointerMode){
        wmfn!(mut self, set_pointer_mode(mode))
    }

    fn is_key_shift_down(state : u32) -> bool {
        static_wmfn!(is_key_shift_down(state))
    }

    fn is_key_ctrl_down(state : u32) -> bool {
        static_wmfn!(is_key_ctrl_down(state))
    }

    fn is_key_alt_down(state : u32) -> bool {
        static_wmfn!(is_key_alt_down(state))
    }

    fn is_key_meta_down(state : u32) -> bool {
        static_wmfn!(is_key_meta_down(state))
    }

    fn is_key_command_down(state : u32) -> bool {
        static_wmfn!(is_key_command_down(state))
    }

    fn is_key_hyper_down(state : u32) -> bool {
        static_wmfn!(is_key_hyper_down(state))
    }

    fn is_capslock_on(state : u32) -> bool {
        static_wmfn!(is_capslock_on(state))
    }

    fn is_numlock_on(state : u32) -> bool {
        static_wmfn!(is_numlock_on(state))
    }

   

    

    


}