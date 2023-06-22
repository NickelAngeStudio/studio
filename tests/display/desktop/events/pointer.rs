use studio::display::desktop::window::Window;
use crate::display::desktop::rsrcs::{SpaceReceiver, main_loop};

/// Test pointer events
pub fn test_pointer(){

   let mut window = Window::new().unwrap();

   window.show();

   main_loop(&mut window, &mut SpaceReceiver::new());

   window.close();


}

/*

  /// Pointer move event. Provides new (x, y) position. Only when in pointer mode.
    Moved((i32, i32)),

    /// Pointer acceleration event.  Provides delta (x, y). Only when in acceleration mode.
    Acceleration((i32, i32)),

    /// Pointer button down event. Provides button number (up to 255) and cursor position (x,y).
    ButtonDown(PointerButton, (i32, i32)),

    /// Pointer button up event. Provides button number (up to 255) and cursor position (x,y).
    ButtonUp(PointerButton, (i32, i32)),


 */