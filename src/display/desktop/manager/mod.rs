//! Implementation of [Window](super::window::Window) according to platform window provider.

use cfg_boost::target_cfg;

use crate::error::StudioError;

use super::{property::{WindowProperty, SubWindowOption, WindowPositionOption, FullScreenMode, PointerMode, KeyboardMode, WindowEventWaitMode}, event::{Event}, Window};

/// Enumeration of [Display server](https://en.wikipedia.org/wiki/Windowing_system#Display_server)
/// and/or [Window manager](https://en.wikipedia.org/wiki/Window_manager) providers.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WindowProvider {

    /// [Wayland](https://en.wikipedia.org/wiki/Wayland_(protocol)) display server.
    Wayland,

    /// [X Window System](https://en.wikipedia.org/wiki/X_Window_System) display server.
    X11,

    /// Microsoft Windows [Desktop Window Manager](https://en.wikipedia.org/wiki/Desktop_Window_Manager) compositing window manager.
    Windows,

    /// Apple MacOS [Quartz](https://en.wikipedia.org/wiki/Quartz_Compositor) compositor.
    MacOs,

    /// [Web assembly](https://en.wikipedia.org/wiki/WebAssembly) browser compositor.
    WASM,
}

target_cfg! {
    linux => {
        // Linux implementation of WindowManager trait
        pub mod linux;
        pub type WindowManagerType<'window> = super::manager::linux::LinuxWindowManager<'window>;
    },

}

/// Manager that manage the [Window].
/// 
/// Each property set returns either true or false depending if window need to be recreate.
pub trait WindowManager<'window> {
    /// Create a new WindowManager instance.
    fn new() -> Result<Self, StudioError> where Self : Sized;

    /// Get the window provider id
    fn get_window_provider(&self) -> WindowProvider;

    /// Get immutable reference to the window properties
    fn get_properties(&self) -> &WindowProperty;

    /// Pop an event from the manager.
    fn poll_event(&mut self) -> &Event;

    /// Recreate window.
    fn recreate(&mut self);

    /// Show the window according to show option.
    fn show(&mut self);

    /// Force close the window.
    fn close(&mut self);

    /// Hide the window.
    fn hide(&mut self);

    /// Restore the window.
    fn restore(&mut self);

    /// Get the OS Window manager window handle.
    fn get_window_handle(&self) -> Option<*const usize>;

    /// Push an event that will be poll during poll_event.
    fn push_event(&self, event: Event);

    target_cfg! {
        linux => {
            /// Get the OS Window manager display handle.
            fn get_display_handle(&self) -> *const usize;
        }
    }

    /// Set window parent.
    fn set_parent<'manager: 'window>(&mut self, parent : &'manager Window<'manager>, option : SubWindowOption) -> bool;

    /// Remove the window parent.
    fn remove_parent(&mut self) -> bool;

    /// Set the window title.
    fn set_title(&mut self, title : &String) -> bool;

    /// Set window absolute position.
    fn set_position(&mut self, option : WindowPositionOption) -> bool;

    /// Set window size.
    fn set_size(&mut self, size : &(u32,u32)) -> bool;

    /// Show window decoration such as title bar, buttons, etc...
    fn show_decoration(&mut self) -> bool;

    /// Hide window decoration such as title bar, buttons, etc...
    fn hide_decoration(&mut self) -> bool;

    /// Minimize window into taskbar.
    fn minimize(&mut self) -> bool;

    /// Maximize window.
    fn maximize(&mut self) -> bool;

    /// Set the [WindowEventWaitMode].
    fn set_event_wait_mode(&mut self, mode : WindowEventWaitMode) -> bool;

    /// Setting window fullscreen trigger 
    /// window recreate.
    fn set_fullscreen(&mut self, fsmode : FullScreenMode) -> bool;

    /// Set keyboard mode.
    fn set_keyboard_mode(&mut self, mode : KeyboardMode) -> bool;

    /// Set the pointer mode.
    fn set_pointer_mode(&mut self, mode : &PointerMode) -> bool;

    /// Set the pointer position relative to the window.
    fn set_pointer_position(&mut self, position : (i32, i32)) -> bool;

    /// Show the pointer, making it visible.
    fn show_pointer(&mut self) -> bool;

    /// Hide the pointer, making it invisible.
    fn hide_pointer(&mut self) -> bool;

    /// Confine the pointer to window boundaries, preventing escape.
    fn confine_pointer(&mut self) -> bool;

    /// Release the pointer from window boundaries, allowing escape.
    fn release_pointer(&mut self) -> bool;

    /*********
    * STATIC *
    *********/
    /// Return true if state indicate either left or right shift were down.
    fn is_key_shift_down(state : u32) -> bool;

    /// Return true if state indicate either left or right ctrl were down.
    fn is_key_ctrl_down(state : u32) -> bool;

    /// Return true if state indicate either left or right alt were down.
    fn is_key_alt_down(state : u32) -> bool;

    /// Return true if state indicate either left or right meta were down.
    /// 
    /// Reference(s)
    /// https://askubuntu.com/questions/19558/what-are-the-meta-super-and-hyper-keys
    fn is_key_meta_down(state : u32) -> bool;

    /// Return true if state indicate either left or right super(command) were down.
    /// 
    /// Reference(s)
    /// https://askubuntu.com/questions/19558/what-are-the-meta-super-and-hyper-keys
    fn is_key_command_down(state : u32) -> bool;

    //// Return true if state indicate either left or right shift hyper down.
    /// 
    /// Reference(s)
    /// https://askubuntu.com/questions/19558/what-are-the-meta-super-and-hyper-keys
    fn is_key_hyper_down(state : u32) -> bool;

    /// Return true if state indicate that capslock was enabled.
    fn is_capslock_on(state : u32) -> bool;

    /// Return true if state indicate that numlock was enabled.
    fn is_numlock_on(state : u32) -> bool;

}