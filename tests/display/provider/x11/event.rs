use std::{rc::Rc, cell::RefCell, process::exit};

/*
use studio::{display::{ error::DisplayError, provider::WindowProvider, desktop::{WINDOW_MIN_WIDTH, WINDOW_MAX_WIDTH, WINDOW_MIN_HEIGHT, WINDOW_MAX_HEIGHT}}, error::StudioError};

use crate::*;



/*********
* CONSTS *
*********/
/// Window dimension
pub const WINDOW_WIDTH:u32 = 320;
pub const WINDOW_HEIGHT:u32 = 240;

/********
* TESTS *
********/
#[test]
#[ignore]   // Should be run manually for control.
/// Create a new X11 Window without error.
/// 
/// # Verification(s)
/// V1 | Window::new(x11) width < WINDOW_MIN_WIDTH should gives StudioError::WindowSizeError.
/// V2 | Window::new(x11) width > WINDOW_MAX_WIDTH should gives StudioError::WindowSizeError.
/// V3 | Window::new(x11) height < WINDOW_MIN_HEIGHT should gives StudioError::WindowSizeError.
/// V4 | Window::new(x11) height > WINDOW_MAX_HEIGHT should gives StudioError::WindowSizeError.
/// V5 | Window::new(x11) created without error.
fn window_x11_new() {
    // V1 | Window::new(x11) width < WINDOW_MIN_WIDTH should gives WindowError::WindowSizeError.
    assert_err!(Window::from_provider(WindowProvider::X11, WINDOW_MIN_WIDTH - 1, WINDOW_HEIGHT), StudioError::Display(DisplayError::SizeError));

    // V2 | Window::new(x11) width > WINDOW_MAX_WIDTH should gives WindowError::WindowSizeError.
    assert_err!(Window::from_provider(WindowProvider::X11,WINDOW_MAX_WIDTH + 1, WINDOW_HEIGHT), StudioError::Display(DisplayError::SizeError));

    // V3 | Window::new(x11) height < WINDOW_MIN_HEIGHT should gives WindowError::WindowSizeError.
    assert_err!(Window::from_provider(WindowProvider::X11,WINDOW_WIDTH, WINDOW_MIN_HEIGHT - 1), StudioError::Display(DisplayError::SizeError));

    // V4 | Window::new(x11) height > WINDOW_MAX_HEIGHT should gives WindowError::WindowSizeError.
    assert_err!(Window::from_provider(WindowProvider::X11,WINDOW_WIDTH, WINDOW_MAX_HEIGHT + 1), StudioError::Display(DisplayError::SizeError));

    // V5 | Window::new(x11) created without error.
    let _kw = assert_ok!(Window::from_provider(WindowProvider::X11,WINDOW_WIDTH, WINDOW_HEIGHT));
}

#[test]
#[ignore]   // Should be run manually for control.
/// Dispatch x11 Window events.
/// 
/// # Verification(s)
/// V1 | Window::dispatch_events() must dispatch without errors.
fn window_x11_dispatch_events() {

    window_x11_prepare!(wx11, dispatcher, receiver, {
        // V1 | Window::dispatch_events() must dispatch without errors.
        loop {
            wx11.dispatch_events(&mut dispatcher, true);
            match receiver.borrow().get_state() {
                crate::display::provider::x11::WindowEventReceiverControlState::Running => {},
                crate::display::provider::x11::WindowEventReceiverControlState::NextStep => break,
                crate::display::provider::x11::WindowEventReceiverControlState::Exit => exit(0),
            }
        }

    });    

}

#[test]
#[ignore]   // Should be run manually for control.
/// Create and run Window in a different thread.
/// 
/// # Verification(s)
/// V1 | Window works in a different thread.
fn window_x11_thread() {
    use std::thread;

    // V1 | Window works in a different thread.
    let thread_join_handle = thread::spawn(move || {

        window_x11_prepare!(wx11, dispatcher, receiver, {
            window_x11_step_loop!(wx11, dispatcher, receiver);
        });    
    });

    let _res = thread_join_handle.join();
}

*/