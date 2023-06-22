//! Ressources functions and macros to run desktop tests.
use std::io::{self, Write};


use std::{process::exit, time::{Duration, self}, thread};

use cfg_boost::target_cfg;
use studio::display::desktop::Window;
use studio::display::desktop::event::Event;
use studio::display::desktop::event::keyboard::{EventKeyboard, KeyIdentity};


use crate::tools::{CYAN_CONSOLE, RESET_CONSOLE, BLUE_CONSOLE, YELLOW_CONSOLE, MAGENTA_CONSOLE};

/************
* CONSTANTS * 
************/
/// Time to wait between main loop cycle, set to 60 FPS as to not stress the cpu too much.
pub const WAIT_MS: Duration = time::Duration::from_millis(1000/60); 


target_cfg! {
    linux => {
        pub const ESC_KEY_VALUE:u32 = 9;        // Escape key value for Linux
        pub const SPACE_KEY_VALUE:u32 = 65;     // Space key value for Linux
    }
}

/*********
* TRAITS *
*********/
/// Event receiver to send events to.
pub trait EventReceiver {
    /// Receive an event
    fn receive(&mut self, event: &Event);

    /// Returning true will break the main loop.
    fn is_test_finished(&self) -> bool;
}

/**********
* STRUCTS *
**********/
/// Receiver that await a space bar to quit.
pub struct SpaceReceiver {
    finished:bool,
}

impl SpaceReceiver {
    pub fn new() -> SpaceReceiver {
        SpaceReceiver { finished: false }
    }
}

impl EventReceiver for SpaceReceiver{
    fn receive(&mut self, event: &Event) {
        match event {
            Event::Keyboard(kb_event) => match kb_event {
                EventKeyboard::KeyDown(key) 
                | EventKeyboard::KeyUp(key) => self.finished = self.finished || ( *key == SPACE_KEY_VALUE ),
                EventKeyboard::KeyPress(key) => println!("Key={:?}", key),
            },
            _ => {},
        }
    }

    fn is_test_finished(&self) -> bool {
        self.finished
    }
}

/************
* FUNCTIONS *
************/
/// Loop window and send events to receiver.
/// Can always be closed using ESC Key.
pub fn main_loop(window: &mut Window, receiver: &mut dyn EventReceiver){

    'main: loop {
          'inner: loop {
            let event = window.poll_event();

            // Send event to receiver.
            receiver.receive(event);

            if receiver.is_test_finished() {
                break 'main;
            }

            match event {
                Event::Keyboard(kb_event) => match kb_event {
                    EventKeyboard::KeyDown(key) => 
                    {      
                        if *key == ESC_KEY_VALUE {
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
        thread::sleep(WAIT_MS);
    }

}


/// Enumeration of input selection
pub enum InputSelection{
    /// Events and inputs tests
    Events,

    /// Properties tests
    Properties,

    /// Method tests
    Methods,

    /// Log window
    LogWindow,

    /// Quit tests
    Quit,

}

/// Print tests instructions since user interaction is needed.
pub fn select_options() -> InputSelection {

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}**{}**{}", CYAN_CONSOLE, "************************", RESET_CONSOLE);
    println!("{}* {} *{}", CYAN_CONSOLE, "WINDOW INTEGRATION TESTS", RESET_CONSOLE);
    println!("{}**{}**{}", CYAN_CONSOLE, "************************", RESET_CONSOLE);

    println!("{}{}{}", BLUE_CONSOLE, "- Blue text is information only.", RESET_CONSOLE);
    print!("{}{}{}", CYAN_CONSOLE, "- Follow instructions in ", RESET_CONSOLE);
    print!("{}{}{}", YELLOW_CONSOLE, "YELLOW", RESET_CONSOLE);
    print!("{}{}{}", CYAN_CONSOLE, ".\n", RESET_CONSOLE);
    println!("{}{}{}", CYAN_CONSOLE, "- Press ESC to exit and fail any test.", RESET_CONSOLE);
    println!("{}{}{}", MAGENTA_CONSOLE, "- Important informations are in MAGENTA.\n", RESET_CONSOLE);
    println!("{}{}{}", BLUE_CONSOLE, "- Select `q` to exit any menu.", RESET_CONSOLE);

    println!("{}{}{}", BLUE_CONSOLE, "TEST : (1) Events    (2) Properties    (3) Methods", RESET_CONSOLE);
    println!("{}{}{}", BLUE_CONSOLE, "OTHER : (p) Open window and log each event.", RESET_CONSOLE);

    let mut selection: Option<InputSelection> = Option::None;

    loop {
        match get_user_selection().as_str() {
            "1" => selection = Some(InputSelection::Events),
            "2" => selection = Some(InputSelection::Properties),
            "3" => selection = Some(InputSelection::Methods),
            "p" => selection = Some(InputSelection::LogWindow),
            "q" => selection = Some(InputSelection::Quit),
            _ => println!("{}{}{}", MAGENTA_CONSOLE, "Invalid selection!" , RESET_CONSOLE),
        }

        if selection.is_some() {
            break;
        }
    }

    selection.unwrap()

}

/// Get user menu selection
pub fn get_user_selection() -> String{
    print!("{}{}{}", YELLOW_CONSOLE, "Make your selection : ", RESET_CONSOLE);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");

    String::from(input.trim())
}

/// Print end of tests
pub fn print_instructions_footer() {
    println!("{}{}{}", MAGENTA_CONSOLE, "Tests finished, focus console and press ENTER to close.", RESET_CONSOLE);
    std::io::stdin().read_line(&mut String::new()).unwrap();
}