use std::panic::catch_unwind;

use crate::kleio::display::{Display, event::KEvent, KFullscreenMode};

use self::bind::{wl_display_connect, wl_display_disconnect};

use super::x11::event::{Display, Window};




/// Waylind C function binds
#[allow(unused)]                    // Remove unused variable notification
#[allow(non_upper_case_globals)]    // Imported C global aren't formatted according to convention.
#[allow(non_camel_case_types)]    // Imported C global aren't formatted according to convention.
pub mod bind;

/// Contains Wayland screen fetch function
pub mod screen;

/// Implementation of privates elements relatives to Wayland display server
#[doc(hidden)]
impl Display {

    // Pop an event from the queue
    #[inline(always)]
    pub(super) fn wayland_poll_event(&mut self) -> KEvent {
        todo!()
    }

    // Sync an event from the queue
    #[inline(always)]
    pub(super) fn wayland_sync_events(&self) {
        todo!()
    }

    /// Get the count of events that need handling.
    #[inline(always)]
    pub(super) fn wayland_get_event_count(&self) -> usize {
        todo!()
    }

    /// Set the cursor position
    #[inline(always)]
    pub(super) fn wayland_set_cursor_position(&mut self, position : (i32, i32)){
        todo!()
    }

    /// Confine cursor to window, preventing it from exiting boundaries.
    #[inline(always)]
    pub fn wayland_confine_cursor(&mut self) {
        todo!()
    }

    /// Release cursor from window, allowing it to exit boundaries.
    #[inline(always)]
    pub fn wayland_release_cursor(&mut self) {
        todo!()
    }

    /// Hide system default cursor.
    #[inline(always)]
    pub fn wayland_hide_cursor(&mut self) {
        todo!()
    }

    /// Show system default cursor.
    #[inline(always)]
    pub fn wayland_show_cursor(&mut self) {
        todo!()
    }

    /// Restore the [Display], undoing any minimized, maximized and/or fullscreen status.
    #[inline(always)]
    pub fn wayland_restore(&mut self) {
        todo!()
    }

    /// Set a new title for the [Display].
    #[inline(always)]
    pub(super) fn wayland_set_title(&mut self) {
        todo!()
    }

    /// Set the [Display] as fullscreen.
    #[inline(always)]
    pub(super) fn wayland_set_fullscreen(&mut self, mode : KFullscreenMode) {
        todo!()
    }

    /// Set a size of [Display].
    #[inline(always)]
    pub(super) fn wayland_set_size(&mut self) {
        todo!()
    }

    /// Set a position of [Display].
    #[inline(always)]
    pub(super) fn wayland_set_position(&mut self) {
        todo!()
    }
  
    /// Get if Wayland is supported.
    #[inline(always)]
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

    /// Create connection to Wayland and window
    #[inline(always)]
    pub(crate) fn create_wayland_window(width:u32, height:u32) -> (*mut Display, *mut Window) {
        todo!()
    }
}