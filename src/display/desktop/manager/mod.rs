//! Implementation of [Window](super::window::Window) according to platform window provider.

use cfg_boost::target_cfg;

use crate::error::StudioError;

use super::{property::{WindowProperty, WindowPositionOption, FullScreenMode, PointerMode, KeyboardMode}, event::{Event}, window::WindowShowOption, Window};

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
        pub type WindowManagerType = super::manager::linux::LinuxWindowManager;
    },

}

/// Manager that manage the [Window].
/// 
/// Each property set returns either true or false depending if window need to be recreate.
pub trait WindowManager {
    /// Create a new WindowManager instance.
    fn new() -> Result<Self, StudioError> where Self : Sized;

    /// Get the window provider id
    fn get_window_provider(&self) -> WindowProvider;

    /// Get immutable reference to the window properties
    fn get_properties(&self) -> &WindowProperty;

    target_cfg! {
        !immediate:ft => {  // Retained mode
            /// Pop an event from the manager.
            fn poll_event(&mut self) -> Event;

            /// Show the window according to show option.
            fn show(&mut self, option : WindowShowOption, parent : Option<&Window>);
        },
        immediate:ft => {   // Immediate mode
            /// Pop an event from the manager and return a reference.
            fn poll_event(&mut self) -> &Event;

            /// Show the window according to show option.
            fn show(&mut self);
        }
    }

    

    /// Force close the window.
    fn close(&mut self);

    /// Restore the window.
    fn restore(&mut self);

    /// Push an event that will be poll during poll_event.
    fn push_event(&self, event: Event);

    /// Refresh properties set. Called after Window::set_properties and Window::set_property.
    fn refresh(&mut self);

    /// Set the window title.
    fn set_title(&mut self, title : &String);

    /// Set window absolute position.
    fn set_position(&mut self, option : WindowPositionOption);

    /// Set window size.
    fn set_size(&mut self, size : &(u32,u32));

    /// Show window decoration such as title bar, buttons, etc...
    fn show_decoration(&mut self);

    /// Hide window decoration such as title bar, buttons, etc...
    fn hide_decoration(&mut self);

    /// Minimize window into taskbar.
    fn minimize(&mut self);

    /// Maximize window.
    fn maximize(&mut self);

    /// Set the window fullscreen according to mode.
    fn set_fullscreen(&mut self, fsmode : FullScreenMode);

    /// Set keyboard mode.
    fn set_keyboard_mode(&mut self, mode : KeyboardMode);

    /// Set keyboard auto repeat.
    fn set_keyboard_auto_repeat(&mut self, auto_repeat : bool);

    /// Set the pointer mode.
    fn set_pointer_mode(&mut self, mode : &PointerMode);

    /// Set the pointer position relative to the window.
    fn set_pointer_position(&mut self, position : (i32, i32));

    /// Show the pointer, making it visible.
    fn show_pointer(&mut self);

    /// Hide the pointer, making it invisible.
    fn hide_pointer(&mut self);

    /// Confine the pointer to window boundaries, preventing escape.
    fn confine_pointer(&mut self);

    /// Release the pointer from window boundaries, allowing escape.
    fn release_pointer(&mut self);

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