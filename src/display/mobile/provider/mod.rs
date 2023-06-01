use cfg_boost::target_cfg;

/// Enumeration of [Display server](https://en.wikipedia.org/wiki/Windowing_system#Display_server)
/// and/or [Window manager](https://en.wikipedia.org/wiki/Window_manager) providers.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WindowProvider {
    /// Android [SurfaceFlinger](https://en.wikipedia.org/wiki/Windowing_system#SurfaceFlinger) compositor.
    Android,

    /// Apple IOS [Quartz](https://en.wikipedia.org/wiki/Quartz_Compositor) compositor.
    IOS,
}
