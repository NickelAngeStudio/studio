use studio::display::desktop::{window::Window, pointer::PointerMode};

use crate::tools::{RESET_CONSOLE, GREEN_CONSOLE, BLUE_CONSOLE};


/*********
* MACROS *
*********/
/// Create a main loop around events
macro_rules! window_events_loop {

    ($window:ident, $event:ident, $match_body:block) => {{
        'main: loop {
            'events: loop {
                let event = $window.poll_event();
    
                match event {
                    $match_body
                    /*
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
                    */
    
                    Event::None => break 'events,     // Break inner loop
                    _   => {},
                }
            }
        }
        
    }};
}



/********
* TESTS * 
********/
/// Get Window event count.
/// 
/// # Verification(s)
/// V1 | Window::get_event_count() returns the event count without error.
pub fn window_get_event_count(window: &mut dyn Window) {

    // V1 | Window::get_event_count() returns the event count without error.
    let _c = window.get_event_count();
}


/// Get and set Window pointer properties.
/// 
/// # Verification(s)
/// V1 | Window::get_cursor_mode() returns the default motion mode.
/// V2 | Window::is_cursor_confined() is false by default.
/// V3 | Window::is_cursor_visible() is true by default.
/// V4 | Window::hide_cursor() hide cursor without error.
/// V5 | Calling Window::hide_cursor() again doesn't generate error.
/// V6 | Window::is_cursor_visible() is false.
/// V7 | Window::show_cursor() show cursor without error.
/// V8 | Calling Window::show_cursor() again doesn't generate error.
/// V9 | Window::is_cursor_visible() is true.
/// V10 | Window::confine_cursor() prevent cursor from exiting boundaries without error.
/// V11 | Calling Window::confine_cursor() again doesn't generate error.
/// V12 | Window::is_cursor_confined() is true.
/// V13 | Window::get_pointer_mode() to acceleration keep the cursor in the middle of window.
/// V14 | Window::get_pointer_mode() returns acceleration.
/// V15 | Window::get_pointer_mode() to pointer release the cursor from the middle of window.
/// V16 | Window::get_pointer_mode() returns pointer.
/// V17 | Window::release_cursor() let cursor exit boundaries without error.
/// V18 | Calling Window::release_cursor() again without error.
/// V19 | Window::is_cursor_confined() is false.
/// V20 | Make cursor hidden. Exiting window must make the cursor reappear and disappear when reentering window.
/// V21 | Make cursor confined. Losing focus should release cursor while gaining focus should confine cursor.
/// V22 | Make cursor mode acceleration. Losing focus should release cursor while gaining focus should confine cursor in center.
pub fn window_pointer_properties(window: &mut dyn Window) {


        // V1 | Window::get_motion_mode() returns the default motion mode.
        assert_eq!(window.get_pointer_properties().mode, PointerMode::Pointer, "Wrong default cursor mode!");

        // V2 | Window::is_cursor_confined() is false by default.
        assert_eq!(window.get_pointer_properties().is_confined, false, "Cursor shouldn't be confined by default!");

        // V3 | Window::is_cursor_visible() is true by default.
        assert_eq!(window.get_pointer_properties().is_visible, true, "Cursor should be visible by default!");

        window_events_loop!()

        window_x11_step_loop!("Cursor should be visible and not confined...", window);

        // V4 | Window::hide_cursor() hide cursor without error.
        window.hide_pointer();

        window_x11_step_loop!("Cursor should be hidden and not confined...", window);

        // V5 | Calling Window::hide_cursor() again doesn't generate error.
        window.hide_pointer();

        // V6 | Window::is_cursor_visible() is false.
        assert_eq!(window.get_pointer_properties().is_visible, false, "Cursor shouldn't be visible!");

        // V7 | Window::show_cursor() show cursor without error.
        window.show_pointer();
        window_x11_step_loop!("Cursor should be visible and not confined...", window);

        // V8 | Calling Window::show_cursor() again doesn't generate error.
        window.show_pointer();

        // V9 | Window::is_cursor_visible() is true.
        assert_eq!(window.get_pointer_properties().is_visible, true, "Cursor should be visible!");

        // V10 | Window::confine_cursor() prevent cursor from exiting boundaries without error.
        window.confine_pointer();
        window_x11_step_loop!("Cursor should be visible and confined...", window);

        // V11 | Calling Window::confine_cursor() again doesn't generate error.
        window.confine_pointer();

        // V12 | Window::is_cursor_confined() is true.
        assert_eq!(window.get_pointer_properties().is_confined, true, "Cursor should confined!");

        // V13 | Window::set_cursor_mode() to acceleration keep the cursor in the middle of window.
        window.set_pointer_mode(PointerMode::Acceleration);

        // V14 | Window::get_cursor_mode() returns acceleration.
        assert_eq!(window.get_pointer_properties().mode, PointerMode::Acceleration, "Cursor mode should be Acceleration!");
        window_x11_step_loop!("Cursor should be visible, confined and stuck in center...", window);

        // V15 | Window::set_cursor_mode() to pointer release the cursor from the middle of window.
        window.set_pointer_mode(PointerMode::Pointer);

        // V16 | Window::get_cursor_mode() returns pointer.
        assert_eq!(window.get_pointer_properties().mode, PointerMode::Pointer, "Cursor mode should be Pointer!");
        window_x11_step_loop!("Cursor should be visible, confined and free to move...", window);

        // V17 | Window::release_cursor() let cursor exit boundaries without error.
        window.release_pointer();
        window_x11_step_loop!("Cursor should be visible, released and free to move...", window);

        // V18 | Calling Window::release_cursor() again without error.
        window.release_pointer();

        // V19 | Window::is_cursor_confined() is false.
        assert_eq!(window.get_pointer_properties().is_confined, false, "Cursor shouldn't confined!");

        // V20 | Make cursor hidden. Exiting window must make the cursor reappear and disappear when reentering window.
        window.hide_pointer();
        window_x11_step_loop!("Exiting window must make the cursor reappear and disappear when reentering window...", window);

        // V21 | Make cursor confined. Losing focus should release cursor while gaining focus should confine cursor.
        window.show_pointer();
        window.confine_pointer();
        window_x11_step_loop!("Losing focus should release cursor while gaining focus should confine cursor...", window);

        // V22 | Make cursor mode acceleration. Losing focus should release cursor while gaining focus should confine cursor in center.
        window.set_pointer_mode(PointerMode::Acceleration);
        window_x11_step_loop!("Losing focus should release cursor while gaining focus should confine cursor in center...", window);

}

