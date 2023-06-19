use crate::{kleio::display::screen::{KScreen}, error::{StudioError, DisplayError}};

/// Private function that fetch Wayland display server screens.
pub(crate) fn get_wayland_screen() -> Result<Vec<KScreen>, StudioError>{
    Err(StudioError::Display(DisplayError::ScreenDetailError))
}