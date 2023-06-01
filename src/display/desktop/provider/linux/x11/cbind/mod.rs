
// Contains X11 functions
pub mod functs;

// Contains X11 constants
#[allow(unused)]                    // Remove unused variable notification
#[allow(non_upper_case_globals)]    // Imported C global aren't formatted according to convention.
pub mod constants;

// Contains x11 window attributes
pub mod attributes;

// Contains x11 structs definition.
#[allow(unused)]                    // Remove unused variable notification
#[allow(non_snake_case)]            // Imported C members aren't formatted according to convention.
pub mod structs;