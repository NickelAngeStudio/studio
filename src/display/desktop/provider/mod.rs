//! Implementation of [Window](super::window::Window) according to platform window provider.

use cfg_boost::target_cfg;

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
        // Linux implementation of Window trait
        pub mod linux;
    },

}