//! Linux implementations of [WindowManager].

use crate::{display::{ desktop::{ event::Event, property::{WindowProperty}, manager::WindowManager}, DisplayError}, error::StudioError};
use self::{wayland::WaylandWindowManager, x11::X11WindowManager};
use super::WindowProvider;

/// Wayland DisplayManager
pub mod wayland;

/// X11 DisplayManager
pub mod x11;


/// Enumeration of implemented [WindowManager]
enum ImplementedLinuxWindowManager{
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

pub struct LinuxWindowManager {
    wm : ImplementedLinuxWindowManager,
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

    #[inline(always)]
    fn poll_event(&mut self) -> Event  {
        wmfn!(mut self, poll_event())
    }

    fn push_event(&mut self, event: Event){
        wmfn!(mut self, push_event(event))
    }

    #[inline(always)]
    fn show(&mut self, property : &WindowProperty) {
        wmfn!(mut self, show(property))
    }

    #[inline(always)]
    fn close(&mut self) {
         wmfn!(mut self, close());
    }

    #[inline(always)]
    fn hide(&mut self) {
         wmfn!(mut self, hide());
    }

    #[inline(always)]
    fn set_title(&mut self, title : &String) -> bool {
         wmfn!(mut self, set_title(title))
    }

    #[inline(always)]
    fn set_position(&mut self, position : (i32,i32)) -> bool {
         wmfn!(mut self, set_position(position))
    }

    #[inline(always)]
    fn set_size(&mut self, size : &(u32,u32)) -> bool {
         wmfn!(mut self, set_size(size))
    }

    #[inline(always)]
    fn show_decoration(&mut self) -> bool {
         wmfn!(mut self, show_decoration())
    }

    #[inline(always)]
    fn hide_decoration(&mut self) -> bool {
         wmfn!(mut self, hide_decoration())
    }

    #[inline(always)]
    fn minimize(&mut self) -> bool {
         wmfn!(mut self, minimize())
    }

    #[inline(always)]
    fn maximize(&mut self) -> bool {
         wmfn!(mut self, maximize())
    }

    #[inline(always)]
    fn enable_autorepeat(&mut self) -> bool {
         wmfn!(mut self, enable_autorepeat())
    }

    #[inline(always)]
    fn disable_autorepeat(&mut self) -> bool {
         wmfn!(mut self, disable_autorepeat())
    }

    #[inline(always)]
    fn set_pointer_position(&mut self, position : &(i32, i32)) -> bool {
         wmfn!(mut self, set_pointer_position(position))
    }

    #[inline(always)]
    fn show_pointer(&mut self) -> bool {
         wmfn!(mut self, show_pointer())
    }

    #[inline(always)]
    fn hide_pointer(&mut self) -> bool {
         wmfn!(mut self, hide_pointer())
    }

    #[inline(always)]
    fn confine_pointer(&mut self) -> bool {
         wmfn!(mut self, confine_pointer())
    }

    #[inline(always)]
    fn release_pointer(&mut self) -> bool {
         wmfn!(mut self, release_pointer())
    }

    #[inline(always)]
    fn get_window_handle(&self) -> Option<*const usize> {
        wmfn!(self, get_window_handle())
    }  

    #[inline(always)]
    fn get_display_handle(&self) -> Option<*const usize> {
        wmfn!(self, get_display_handle())
    }

}