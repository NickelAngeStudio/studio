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
        'outer: loop {
            'inner: loop {
                let event = $window.poll_event();

                match event {
                    WindowEvent::Keyboard(event) => match event {
                        WindowEventKeyboard::KeyDown(keycode) => 
                        {        
                            println!("KeyCode={}", keycode);
                            if *keycode == 65 {
                                break 'outer;   // Break outer loop
                            }
        
                            if *keycode == self.exit_key {
                                exit(0);    // Exit test
                            }
                            false
                        },
                        WindowEventKeyboard::KeyUp(_) => { false },
                    },

                    WindowEvent::None => break 'inner,     // Break inner loop
                _   => {},
                }
            }
        }
    }};

    // With message
    ($message:expr, $window:ident) => {{
        println!("\x1b[93m{}\x1b[0m", $message);

        'outer: loop {
            'inner: loop {
                let event = $window.poll_event();

                match event {
                    Event::Keyboard(event) => match event {
                        EventKeyboard::KeyDown(keycode) => 
                        {        
                            println!("KeyCode={}", keycode);
                            if keycode == 65 {
                                break 'outer;   // Break outer loop
                            }
        
                            if keycode == 9 {
                                exit(0);    // Exit test
                            }

                        },
                        EventKeyboard::KeyUp(_) => { },
                    },

                    Event::None => break 'inner,     // Break inner loop
                _   => {},
                }
            }
        }
        //$receiver.borrow_mut().set_state(crate::display::provider::x11::WindowEventReceiverControlState::Running);
    }};
}


/*********
* STRUCT *
*********/
/*
/// Enumeration of WindowEventReceiverControlResult state.
#[derive(Debug, Clone, Copy)]
enum WindowEventReceiverControlState {

    // Test is running
    Running,

    // Tell receiver to get to next step
    NextStep,

    // Exit program
    Exit,
}


/// Receiver used to control test since all tests are runned manually.
/// 
/// Pressing step_key to exit step loop.
/// Pressing Esc will exit the program.
struct WindowEventReceiverControl {
    /// Key to press for next step
    step_key : u32,

    /// Key to press to exit
    exit_key : u32,


    /// State
    state : WindowEventReceiverControlState,
}

impl WindowEventReceiverControl {
    pub fn new(step_key : u32, exit_key : u32) -> WindowEventReceiverControl {
        WindowEventReceiverControl { step_key, exit_key, state : WindowEventReceiverControlState::Running }
    }

    pub fn get_state(&self) -> WindowEventReceiverControlState {
        self.state
    }

    pub fn set_state(&mut self, state : WindowEventReceiverControlState)  {
        self.state = state
    }
}

impl WindowEventReceiver for WindowEventReceiverControl {
    fn handle_event(&mut self, event : &WindowEvent) -> bool {
        match event {
            WindowEvent::Keyboard(event) => match event {
            WindowEventKeyboard::KeyDown(keycode) => 
                {
                    println!("Keydown={}", keycode);
                    self.state = WindowEventReceiverControlState::Running;

                    println!("KeyCode={}", keycode);
                    if *keycode == self.step_key {
                        self.state = WindowEventReceiverControlState::NextStep;
                    }

                    if *keycode == self.exit_key {
                        self.state = WindowEventReceiverControlState::Exit;
                    }
                    false
                },
                WindowEventKeyboard::KeyUp(_) => { false },
            },
            _ => false,
        }
    }

    fn is_enabled(&self) -> bool {
        true
    }
}
*/