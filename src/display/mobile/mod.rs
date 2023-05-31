
/*
impl Window {
    /// Create a new [Display] for mobile devices.
    /// 
    /// Return New [`Display`] created.
    pub fn new() -> Result<Display, StudioError> {        
        todo!()
    }
}


/// Abstraction of a [Display server](https://en.wikipedia.org/wiki/Windowing_system#Display_server)
/// and/or [Window manager](https://en.wikipedia.org/wiki/Window_manager) used to create and manage window.
pub trait WindowManager {
    /// Get the window provider managing this window.
    fn get_window_provider(&self) -> DisplayProvider;

    /// Pop a window event from the queue.
    fn poll_event(&mut self) -> DisplayEvent;

    /// Get the count of events that need handling.
    fn get_event_count(&self) -> usize;

    /// Get windows properties.
    /// 
    /// The [KWindowManager] is responsible for updating this struct.
    fn get_window_property(&self) -> &DisplayProperty;

    /// Restore the window, undoing any minimized, maximized and/or fullscreen status.
    /// 
    /// Must be overridden for desktop implementation.
    fn restore(&mut self)  { todo!( )}

    /// Set a new title for the window.
    fn set_title(&mut self, title : &str);

    /// Perform sync with the display server / window manager.
    fn sync(&self);

    /// Get self as Any, use for downcast. 
    /// 
    /// Implementation only need to return self.
    fn as_any(&self) -> &dyn Any;

    /// Get self as mut Any, use for downcast. 
    /// 
    /// Implementation only need to return mut self.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
*/