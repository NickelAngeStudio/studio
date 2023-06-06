//! Ressources functions and macros to run desktop tests.

use std::{process::exit, time::{Duration, self}, thread};

use cfg_boost::target_cfg;
use studio::display::desktop::{event::{Event, EventKeyboard}, window::Window};

use crate::tools::{CYAN_CONSOLE, RESET_CONSOLE, BLUE_CONSOLE, YELLOW_CONSOLE};

/************
* CONSTANTS * 
************/
/// Time to wait between main loop cycle, set to 60 FPS as to not stress the cpu too much.
pub const WAIT_MS: Duration = time::Duration::from_millis(1000/60); 


target_cfg! {
    linux => {
        pub const ESC_KEY_VALUE:u32 = 9;        // Escape key value for Linux
    }
}

/*********
* TRAITS *
*********/
/// Event receiver to send events to.
pub trait EventReceiver {
    /// Called before receiving events
    fn before_receive(&mut self);

    /// Receive an event
    fn receive(&mut self, event: Event);

    /// Called after receiving events
    fn after_receive(&mut self);

    /// Returning true will break the main loop.
    fn is_test_finished(&self) -> bool;
}

/************
* FUNCTIONS *
************/
/// Loop window and send events to receiver.
/// Can always be closed using ESC Key.
pub fn main_loop(window: &mut dyn Window, receiver: &mut dyn EventReceiver){

    'main: loop {
        receiver.before_receive();
        'inner: loop {
            let event = window.poll_event();

            // Send event to receiver.
            receiver.receive(event);

            match event {
                Event::Keyboard(kb_event) => match kb_event {
                    EventKeyboard::KeyDown(keycode) => 
                    {        
                        if keycode == ESC_KEY_VALUE {
                            exit(1);    // Exit and fail test
                        }
                    },
                    _ => {},
                },
                Event::None => break 'inner,     // Break inner loop on Event::None;
                _  => {},
            }
        }

        // Break main loop if receiver is done.
        if receiver.is_test_finished() {
            break 'main;
        }
        receiver.after_receive();
        thread::sleep(WAIT_MS);
    }

}

/// Print tests instructions since user interaction is needed.
pub fn print_instructions_header() {

    println!("{}**{}**{}", CYAN_CONSOLE, "************************", RESET_CONSOLE);
    println!("{}* {} *{}", CYAN_CONSOLE, "WINDOW INTEGRATION TESTS", RESET_CONSOLE);
    println!("{}**{}**{}", CYAN_CONSOLE, "************************", RESET_CONSOLE);

    println!("{}{}{}", BLUE_CONSOLE, "1. Blue text is information only.", RESET_CONSOLE);
    print!("{}{}{}", CYAN_CONSOLE, "2. Follow instructions in ", RESET_CONSOLE);
    print!("{}{}{}", YELLOW_CONSOLE, "YELLOW", RESET_CONSOLE);
    print!("{}{}{}", CYAN_CONSOLE, ".\n", RESET_CONSOLE);
    println!("{}{}{}", CYAN_CONSOLE, "3. Press ESC to exit and fail any test.", RESET_CONSOLE);

    println!("{}{}**{}", BLUE_CONSOLE, "\n*************", RESET_CONSOLE);
    println!("{}* {} *{}", BLUE_CONSOLE, "TESTS START", RESET_CONSOLE);
    println!("{}**{}**{}", BLUE_CONSOLE, "***********", RESET_CONSOLE);


}

/// Print end of tests
pub fn print_instructions_footer() {
    println!("{}{}**{}", BLUE_CONSOLE, "\n***********", RESET_CONSOLE);
    println!("{}* {} *{}", BLUE_CONSOLE, "TESTS END", RESET_CONSOLE);
    println!("{}**{}**{}", BLUE_CONSOLE, "*********", RESET_CONSOLE);
}