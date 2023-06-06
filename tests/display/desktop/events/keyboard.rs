use std::{process::exit, time::Instant};

use cfg_boost::target_cfg;
use studio::display::desktop::{window::Window, event::{EventKeyboard, Event}};

use crate::{display::desktop::{ rsrcs::{EventReceiver, main_loop}}, tools::{YELLOW_CONSOLE, RESET_CONSOLE, BLUE_CONSOLE}};


target_cfg! {
    linux => {
        pub const SPACE_KEY_VALUE:u32 = 65;     // Space key value for Linux
    }
}

/// Test keyboard events
pub fn test_keyboard(window: &mut dyn Window){

    println!("{}{}{}", BLUE_CONSOLE, "Starting keyboard event tests ...", RESET_CONSOLE);
    
    // Hold spacebar test
    main_loop(window, &mut HoldSpace::new());

    println!("{}{}{}", BLUE_CONSOLE, "... keyboard event tests ended ...", RESET_CONSOLE);
}

/// Struct that test holding space bar as an Event Receiver.
struct HoldSpace {
    is_done: bool,          // Is the test finished.
    duration : Instant,     // Duration of press
    space_pressed : bool,   // True if bar is pressed, false otherwise
    step_done:bool,         // Is holding space step done.
    step_msg:bool,          // If 2nd step message has showed.
}

impl HoldSpace {
    pub fn new() -> HoldSpace {
        println!("{}Hold SPACE on keyboard until told to release...{}", YELLOW_CONSOLE, RESET_CONSOLE);
        HoldSpace { is_done: false, duration: Instant::now(), space_pressed: false, step_done: false, step_msg: false }
    }

    /// Step where user hold space
    #[inline(always)]
    pub(super) fn hold_space_step(&mut self, kb_event : EventKeyboard){
        
        match kb_event {
            EventKeyboard::KeyDown(keycode) => {
                if keycode == SPACE_KEY_VALUE {
                    println!("{}{}{}", BLUE_CONSOLE, "Space is now down ...", RESET_CONSOLE);
                    self.space_pressed = true;
                    self.duration = Instant::now();
                }
            },
            EventKeyboard::KeyUp(keycode) => {
                if keycode == SPACE_KEY_VALUE {
                    println!("{}{}{}", BLUE_CONSOLE, "Space is released too soon, try again ...", RESET_CONSOLE);
                    self.space_pressed = false;
                }
            },
        }

        if self.space_pressed {
            if self.duration.elapsed().as_secs() >= 2 { // Space was hold for 2 seconds.
                self.step_done = true;
            }
        }
    }

    /// Step where user release space.
    #[inline(always)]
    pub(super) fn release_space_step(&mut self, kb_event : EventKeyboard) {
        if !self.step_msg { // Show next step message.
            println!("{}Release SPACE...{}", YELLOW_CONSOLE, RESET_CONSOLE);
            self.step_msg = true; 
        }

        // Verify if space key was released.
        if let EventKeyboard::KeyUp(keycode) = kb_event {
            if keycode == SPACE_KEY_VALUE {
                self.is_done = true;
            }
        }
        

    }
}

impl EventReceiver for HoldSpace {
    fn receive(&mut self, event: Event) {

        // Only validate keyboard events
        if let Event::Keyboard(kb_event) = event {
            if !self.step_done {
                self.hold_space_step(kb_event);
            } else {
                self.release_space_step(kb_event);
            }
        }
    }

    fn is_test_finished(&self) -> bool {
        self.is_done
    }

    fn before_receive(&mut self) {
        if !self.step_done {
            self.hold_space_step(EventKeyboard::KeyDown(0));
        } else {
            self.release_space_step(EventKeyboard::KeyDown(0));
        }
    }

    fn after_receive(&mut self) {}
}