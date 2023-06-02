use std::process::exit;

use studio::display::desktop::{window::Window, event::{Event, EventKeyboard}};

/// X11 Window Manager property tests
pub mod property;

/// X11 Window Event tests
pub mod event;

/*********
* MACROS *
*********/
/// Macro used to create Window, dispatcher and control receiver.
/// Keys : Next=Space, Exit=ESC
#[macro_export]
macro_rules! window_x11_prepare {

    // This macro call doesn't create KAssetSourceFolder nor the files
    ($window:ident $test_body:block) => {{
        // Create Window
        #[allow(unused_mut)]
        let mut $window = assert_ok!(get_x11_window(WINDOW_WIDTH, WINDOW_HEIGHT));

        // Create dispatcher
        //#[allow(unused_mut)]
        //let mut $dispatcher = studio::display::event::dispatcher::WindowEventDispatcher::new(true);

        // Create and add receiver to dispatcher
        //let $receiver = Rc::new(RefCell::new(crate::display::provider::x11::WindowEventReceiverControl::new(65, 9)));
        //match $dispatcher.add_event_receiver($receiver.clone()){
        //    Ok(_) => {},
        //    Err(_) => panic!("Receiver error!"),
        //}

        // Test body
        $test_body

        // Last wait loop
        window_x11_step_loop!("End of test, press SPACE to exit...", $window);
    }};
}

/// Loop until next step key is pressed.
#[macro_export]
macro_rules! window_x11_step_loop {

    // Without message
    ($window:ident) => {{
        dispatch_events(&mut $window);
    }};

    // With message
    ($message:expr, $window:ident) => {{
        println!("\x1b[93m{}\x1b[0m", $message);

        dispatch_events(&mut $window);
    }};
}

/************
* FUNCTIONS *
************/
pub fn dispatch_events(window: &mut dyn Window){
    'outer: loop {
        'inner: loop {
            let event = window.poll_event();

            match event {
                Event::Keyboard(event) => match event {
                    EventKeyboard::KeyDown(keycode) => 
                    {        
                        if keycode == 65 {
                            break 'outer;   // Break outer loop
                        }
    
                        if keycode == 9 {
                            exit(0);    // Exit test
                        }
                    },
                    EventKeyboard::KeyUp(_) => {},
                },

                Event::None => break 'inner,     // Break inner loop
            _   => {},
            }
        }
    }
}