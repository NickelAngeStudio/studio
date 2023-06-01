use crate::display::error::DisplayError;

/// Union of all possibles errors that might occurs within Studio.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum StudioError {

    /// Error that happens within display module. 
    Display(DisplayError),

    

}