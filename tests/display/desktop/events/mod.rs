
use crate::{tools::{BLUE_CONSOLE, RESET_CONSOLE, MAGENTA_CONSOLE}, display::desktop::rsrcs::get_user_selection};
use self::{keyboard::test_keyboard, pointer::test_pointer};

/**
 * This module test all events.
 */

pub fn window_events_tests() {

    loop {
        println!("{}{}{}", BLUE_CONSOLE, "EVENTS : (1) Keyboard    (2) Pointer    (3) Window    (4) Gamepad", RESET_CONSOLE);

        match get_user_selection().as_str() {
            "1" => test_keyboard(),
            "2" => test_pointer(),
            "3" => todo!(),
            "4" => todo!(),
            "q" => break,
            _ => println!("{}{}{}", MAGENTA_CONSOLE, "Invalid selection!" , RESET_CONSOLE),
        }

    }
}

// Keyboard event tests
mod keyboard;

// Mouse event tests
mod pointer;

// Gamepad event tests
mod gamepad;

// Window event tests
mod window;







