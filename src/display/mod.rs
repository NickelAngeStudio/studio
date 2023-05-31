use cfg_boost::{target_cfg, match_cfg};

use crate::error::StudioError;

use self::desktop::window::Window;

/// Enumeration of possible display errors.
pub mod error;

/// [DisplayProvider] definition.
#[doc(hidden)]
pub mod provider;

/// Hardware screen details and supported resolutions.
pub mod screen;

/// Elements relatives to [Display] events and handling.
pub mod event;


target_cfg! {
    desktop => {       
        /// Desktop implementation of display
        pub mod desktop;

        /// Create a window from width and height
        pub fn create_window(width: u32, height: u32) -> Result<impl Window, StudioError>  {
            match_cfg! {
                linux => {
                    provider::linux::get_linux_window(width, height)
                },
                _ => todo!()
            }
        }
    },

    mobile => {
        /// Mobile implementation of display
        pub mod mobile;
    }
}


/*
/// Create and manage a surface to display. 
/// 
/// [Display] broadcasts [DisplayEvent] to multiple [DisplayEventReceiver] via [Display::dispatch_events()].
/// 
/// Act as a window factory.
/// 
/// TODO: More doc about OS, dispatch, and Examples
pub struct Window {

    /// Display event dispatcher
    dispatcher : WindowEventDispatcher,

    /// [DisplayManager] that manage the display.
    manager : Box<dyn WindowManager>,
}

impl Window {

    pub fn get_window_provider(&self) -> WindowProvider {
        self.manager.get_window_provider()
    }

    pub fn get_window_manager(&self) -> &Box<dyn WindowManager>{
        &self.manager
    }

    /// Dispatch [DisplayEvent] to [DisplayEventReceiver] using a [DisplayEventDispatcher].
    /// 
    /// # Note(s)
    /// After dispatching events, [KWindow::sync()] will be called automatically.
    /// 
    /// # Example(s)
    /// Dispatching at each loop call in Main loop
    /// ```
    /// // Create a KWindow
    /// let mut w = KWindow::new(100,100,100,100,true);
    /// 
    /// // Create a dispatcher that doesn't log unhandled events.
    /// let mut ked = DisplayEventDispatcher::new(false);
    /// 
    /// ... add receivers via ked.add_receiver() ...
    /// 
    /// loop {
    ///     // Dispatch events
    ///     w.dispatch_events(&mut ked);
    /// }
    /// ```
    pub fn dispatch_events(&mut self, dispatcher : &mut WindowEventDispatcher, sync : bool) {

        // First get the event count to poll. This is important to prevent bloking.
        let event_count = self.manager.get_event_count();

        for _ in 0..event_count {
            // Fetch event
            let event = self.manager.poll_event();

            println!("EVENT={:?}", event);

            match event {
                WindowEvent::None => {}, // None event are not dispatched
                _ => {
                    dispatcher.dispatch(&event);
                },
            }
        }

        // Sync events with window manager
        if sync {
            self.manager.sync();
        }
    }

    /// Returns a list of connected screens with details.
    pub fn get_screen_list(&self) -> Result<ScreenList, StudioError> {

        ScreenList::from_provider(self.manager.get_window_provider())

    }

    /// Sync all event between client and display server / window manager. 
    /// 
    /// This need to be called at each loop if [KWindow::dispatch_events()] sync parameter = false..
    pub fn sync(&self) {
        self.manager.sync();
    }

}
*/

