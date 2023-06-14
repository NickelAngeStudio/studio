use std::{rc::Rc, cell::RefCell};

use cfg_boost::target_cfg;
use studio::display::desktop::{window::Window, property::{WindowPropertySet, KeyboardPropertySet}};

use crate::{display::desktop::{ rsrcs::{main_loop}}, tools::{RESET_CONSOLE, BLUE_CONSOLE}};


target_cfg! {
    linux => {
        pub const SPACE_KEY_VALUE:u32 = 65;     // Space key value for Linux
    }
}

/// Test keyboard events
pub fn test_keyboard(window: Rc<RefCell<Window>>){

    println!("{}{}{}", BLUE_CONSOLE, "Starting keyboard event tests ...", RESET_CONSOLE);
    
    // Hold spacebar test
    main_loop(window.clone(), &mut hold_space::HoldSpace::new());

    // Different keys test
    main_loop(window.clone(), &mut different_keys::DifferentKeys::new());

    // Hold different keys
    main_loop(window.clone(), &mut hold_different_keys::HoldDifferentKeys::new());

    // Auto-repeat
    window.borrow_mut().set_property(WindowPropertySet::Keyboard(KeyboardPropertySet::EnableAutoRepeat)).expect("");
    main_loop(window.clone(), &mut auto_repeat_space::AutoRepeatSpace::new());
    window.borrow_mut().set_property(WindowPropertySet::Keyboard(KeyboardPropertySet::DisableAutoRepeat)).expect("");
    

    println!("{}{}{}", BLUE_CONSOLE, "... keyboard event tests ended ...", RESET_CONSOLE);


}

/// This module contains the test which user must hold space bar for 5 seconds.
/// 
/// This test is used to see if an anti-repeat routine has been implemented.
mod hold_space {
    use std::time::Instant;

    use studio::display::desktop::event::{Event, EventKeyboard};

    use crate::{display::desktop::rsrcs::EventReceiver, tools::{YELLOW_CONSOLE, RESET_CONSOLE, BLUE_CONSOLE}};

    use super::SPACE_KEY_VALUE;

    const HOLD_TIME_SEC:u64 = 5;  // Count of seconds to hold bar

    /// Struct that test holding space bar as an Event Receiver.
    pub struct HoldSpace {
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
        pub(super) fn hold_space_step(&mut self, event: Event){
            
            // Only validate keyboard events
            if let Event::Keyboard(kb_event) = event {
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
            }

            if self.space_pressed {
                if self.duration.elapsed().as_secs() >= HOLD_TIME_SEC { // Space was hold for 5 seconds.
                    self.step_done = true;
                }
            }
        }

        /// Step where user release space.
        #[inline(always)]
        pub(super) fn release_space_step(&mut self, event: Event) {
            if !self.step_msg { // Show next step message.
                println!("{}Release SPACE...{}", YELLOW_CONSOLE, RESET_CONSOLE);
                self.step_msg = true; 
            }

            if let Event::Keyboard(kb_event) = event {
                // Verify if space key was released.
                if let EventKeyboard::KeyUp(keycode) = kb_event {
                    if keycode == SPACE_KEY_VALUE {
                        self.is_done = true;
                    }
                }
            }
            

        }
    }

    impl EventReceiver for HoldSpace {

    fn receive(&mut self, event: Event) {

        if !self.step_done {
            self.hold_space_step(event);
        } else {
            self.release_space_step(event);
        }
        
    }

    fn is_test_finished(&self) -> bool {
        self.is_done
    }

}
}

/// This module contains a test which user must press 50 differents keys.
/// 
/// This test is used to see if the keyboard implementation isn't using generic
/// values.
mod different_keys{
    use std::collections::HashMap;

    use studio::display::desktop::event::{Event, EventKeyboard};

    use crate::{tools::{YELLOW_CONSOLE, RESET_CONSOLE, BLUE_CONSOLE}, display::desktop::rsrcs::EventReceiver};

    // Count of keys to press.
    const KEY_COUNT:usize = 50;

    /// Struct that test pressing multiples differents keys
    pub struct DifferentKeys {
        is_done: bool,          // Is the test finished.
        keymap : HashMap<u32, usize>,
        almost_done_msg:bool,   // Indicate if almost done is printed
    }

    impl DifferentKeys {
        pub fn new() -> DifferentKeys {
            println!("{}Press and release {} differents keys...{}", YELLOW_CONSOLE, KEY_COUNT, RESET_CONSOLE);
            DifferentKeys { is_done: false, keymap: HashMap::new(), almost_done_msg: true }
        }
    }

    impl EventReceiver for DifferentKeys {

        fn receive(&mut self, event: Event) {

            if self.keymap.len() % 10 == 0 && !self.almost_done_msg {   // Tell progression each 10 keys
                println!("{}Thats {} keys, only {} mores!{}", YELLOW_CONSOLE, self.keymap.len(), KEY_COUNT - self.keymap.len(), RESET_CONSOLE);
                self.almost_done_msg = true;
            }

            if let Event::Keyboard(kb_event) = event {
                match kb_event {
                    EventKeyboard::KeyDown(keycode) => {
                        if !self.keymap.contains_key(&keycode) {
                            self.almost_done_msg = false;
                            self.keymap.insert(keycode, 1);
                        } else {
                            println!("{}{}{}", BLUE_CONSOLE, "You already pressed that key!", RESET_CONSOLE);
                        }
                       
                    },
                    EventKeyboard::KeyUp(_) => {},
                }
            }

            if self.keymap.len() >= KEY_COUNT && !self.is_done {
                println!("{}Thats {} keys, you did it!{}", YELLOW_CONSOLE, self.keymap.len(), RESET_CONSOLE);
                self.is_done = true;
            }
            
        }

        fn is_test_finished(&self) -> bool {
            self.is_done
        }

    }
}


/// This module contain a test which user must hold 3 different keys for 5 seconds.
/// 
/// This test is used to see if the anti-repeat routine works with more than 1 key.
mod hold_different_keys{
    use std::{collections::HashMap, time::Instant};

    use studio::display::desktop::event::{Event, EventKeyboard};

    use crate::{tools::{YELLOW_CONSOLE, RESET_CONSOLE, MAGENTA_CONSOLE}, display::desktop::rsrcs::EventReceiver};

    // Count of keys to hold.
    const KEY_COUNT:usize = 3;

    // Count of seconds to hold keys
    const HOLD_TIME_SEC:u64 = 5;  

    /// Struct that test holding space bar as an Event Receiver.
    pub struct HoldDifferentKeys {
        is_done: bool,          // Is the test finished.
        keymap : HashMap<u32, usize>,
        duration : Instant,     // Duration of press
        key_valid:bool,         // True if keys are valid
        print_msg:bool,     // Verify if we print a message
    }

    impl HoldDifferentKeys {
        pub fn new() -> HoldDifferentKeys {
            println!("{}{}{}", MAGENTA_CONSOLE, "IMPORTANT : Some keyboard cannot physicaly handle 3 key pressed on the same circuit. Try to spread the keys you hold!", RESET_CONSOLE);
            HoldDifferentKeys { is_done: false, keymap: HashMap::new(), duration: Instant::now(), print_msg: true, key_valid: false }
        }
    }

    impl EventReceiver for HoldDifferentKeys {

        fn receive(&mut self, event: Event) {

            let keypress = self.keymap.iter().filter(|x| x.1 == &1).count();

            match keypress {
                0 => {
                    if self.print_msg {
                        println!("{}Press and hold {} differents keys...{}", YELLOW_CONSOLE, KEY_COUNT, RESET_CONSOLE);
                        self.print_msg = false;
                    }
                    
                    self.key_valid = false;
                },
                1 => {
                    if self.print_msg {
                        println!("{}Press and hold {} differents keys...{}", YELLOW_CONSOLE, KEY_COUNT - 1, RESET_CONSOLE);
                        self.print_msg = false;
                    }
                    self.key_valid = false;
                },
                2 => {
                    if self.print_msg {
                        println!("{}Press and hold {} differents keys...{}", YELLOW_CONSOLE, KEY_COUNT - 2, RESET_CONSOLE);
                        self.print_msg = false;
                    }
                    self.key_valid = false;
                },
                3 => {
                    if self.print_msg {
                        println!("{}Press and hold those differents keys...{}", YELLOW_CONSOLE, RESET_CONSOLE);
                        self.print_msg = false;
                    }

                    if !self.key_valid  {
                        self.key_valid = true;
                        self.duration = Instant::now();

                        
                    } else {
                        if self.duration.elapsed().as_secs() >= HOLD_TIME_SEC { // Space was hold for 5 seconds.
                            self.is_done = true;
                        }
                    }
                },  
                _ => {
                    if self.print_msg {
                        println!("{}Holding too many keys! Only {} differents keys needed...{}", YELLOW_CONSOLE, KEY_COUNT, RESET_CONSOLE);
                        self.print_msg = false;
                    }
                    self.key_valid = false;
                }
            }
            

            if let Event::Keyboard(kb_event) = event {
                self.print_msg = true;
                match kb_event {
                    EventKeyboard::KeyDown(keycode) => {
                        self.keymap.insert(keycode, 1);
                       
                    },
                    EventKeyboard::KeyUp(keycode) => {
                        self.keymap.insert(keycode, 0);
                    },
                }
            }
            
        }

        fn is_test_finished(&self) -> bool {
            if self.is_done {
                println!("{}Perfect, hold different key test is finished!{}", YELLOW_CONSOLE, RESET_CONSOLE);
            }

            self.is_done
        }

    }
}


/// This module contains the auto-repeat test which disable anti-repeat routine.
/// 
/// This test is used to see auto-repeat can be enabled.
mod auto_repeat_space {
    use studio::display::desktop::event::{Event, EventKeyboard};

    use crate::{display::desktop::rsrcs::EventReceiver, tools::{YELLOW_CONSOLE, RESET_CONSOLE}};

    use super::SPACE_KEY_VALUE;

    const SPACE_BAR_COUNT:usize = 300;  // Count of spacebar press needed to finish (about 5 secs)

    /// Struct that test holding space bar as an Event Receiver.
    pub struct AutoRepeatSpace {
        is_done: bool,          // Is the test finished.
        press_count:usize,      // Count of spacebar press
    }

    impl AutoRepeatSpace {
        pub fn new() -> AutoRepeatSpace {
            println!("{}Hold SPACE on keyboard for auto-repeat test...{}", YELLOW_CONSOLE, RESET_CONSOLE);
            AutoRepeatSpace { is_done: false, press_count: 0 }
        }
    }

    impl EventReceiver for AutoRepeatSpace {

    fn receive(&mut self, event: Event) {

        if let Event::Keyboard(kb_event) = event {
            // Verify if space key was released.
            if let EventKeyboard::KeyUp(keycode) = kb_event {
                if keycode == SPACE_KEY_VALUE {
                    self.press_count+=1;    // Increment press count
                }
            }
        }

        if self.press_count>=SPACE_BAR_COUNT {
            self.is_done = true;
        }
        
        
    }

    fn is_test_finished(&self) -> bool {
        self.is_done
    }

}
}