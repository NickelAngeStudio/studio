use std::{cell::RefCell, rc::Rc};

use studio::display::desktop::window::Window;

use self::keyboard::test_keyboard;

/**
 * This module test all events.
 */

pub fn window_events_tests(window: Rc<RefCell<Window>>) {
    // Test keyboard events.
    test_keyboard(window);

}

// Keyboard event tests
mod keyboard;

// Mouse event tests
mod mouse;

// Gamepad event tests
mod gamepad;

// Window event tests
mod window;







