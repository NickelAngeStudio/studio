//! Desktop components abstraction of display.
//! 
//! Contains components needed to create and manage a desktop windows.

use cfg_boost::match_cfg;
use super::StudioError;

// Hardware screen list
pub mod screen;

// Pointer module
pub mod pointer;

// Keyboard properties module
pub mod keyboard;

// Window abstraction and properties
pub mod window;

// Window events input such as mouse, keyboard, etc..
pub mod event;

// Window providers
pub mod provider;


/// Create a window from width and height
pub fn create_window(width: u32, height: u32) -> Result<impl window::Window, StudioError>  {
    match_cfg! {
        linux => {
            provider::linux::get_linux_window(width, height)
        },
        _ => todo!()
    }
}