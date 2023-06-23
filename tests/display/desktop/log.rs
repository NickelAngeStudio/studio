//! Log window receiver that output all event received to the console.

use studio::display::desktop::{event::{Event}, Window, property::{WindowPropertySet, WindowEventWaitMode, KeyboardPropertySet, KeyboardMode}};

use crate::{tools::{BLUE_CONSOLE, RESET_CONSOLE}, display::desktop::rsrcs::main_loop};

use super::rsrcs::EventReceiver;

/// Create a window that log all events.
pub fn log_window(){

    let mut window = Window::new().unwrap();
    window.set_properties(&[WindowPropertySet::SetEventWaitMode(WindowEventWaitMode::AlwaysWait),
    WindowPropertySet::Keyboard(KeyboardPropertySet::SetMode(KeyboardMode::TextInput))]).unwrap();

    window.show();

    println!("{}{}{}", BLUE_CONSOLE, "Opening log window...", RESET_CONSOLE);
    
    // Log window main loop
    main_loop(&mut window, &mut LogWindow::new());
    

    println!("{}{}{}", BLUE_CONSOLE, "... closing log window", RESET_CONSOLE);

}

pub struct LogWindow {

}

impl LogWindow {
    pub fn new() -> LogWindow {
        LogWindow {  }
    }
}

impl EventReceiver for LogWindow {
    fn receive(&mut self, event: &Event) {
        match event {
            Event::None => {},  // Don't print None event.
            _ =>  println!("{:?}", event),
        }
       
    }

    fn is_test_finished(&self) -> bool {
        false
    }
}