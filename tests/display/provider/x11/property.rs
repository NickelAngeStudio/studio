use core::time;
use std::thread;
use std::time::Duration;
use std::process::exit;

use studio::display::desktop::pointer::PointerMode;
use studio::display::desktop::window::Window;
use studio::display::error::DisplayError;
use studio::display::desktop::event::{ Event, EventKeyboard};
use studio::display::desktop::provider::WindowProvider;
use studio::display::desktop::provider::linux::get_x11_window;
use studio::display::desktop::provider::linux::x11::X11Window;
use studio::error::StudioError;

use crate::*;

/*********
* CONSTS *
*********/
/// Window dimension
pub const WINDOW_WIDTH:u32 = 640;
pub const WINDOW_HEIGHT:u32 = 480;

/// New position for cursor tests. Must be center of Window.
pub const CURSOR_X:i32 = 320;
pub const CURSOR_Y:i32 = 240;

/// New position for Window.
pub const WINDOW_POS_X:i32 = 151;
pub const WINDOW_POS_Y:i32 = 262;

/// New title for Window with special characters
pub const WINDOW_TITLE : &str = "*Test window title çéàè*&?%!";

/// Time to wait between stress cycle
pub const WAIT_MS: Duration = time::Duration::from_millis(1);

/********
* TESTS *
********/
#[test]
#[ignore = "User interaction"]
/// Get X11 Window display server elements.
/// 
/// # Verification(s)
/// V1 | Window::get_display_server_provider() returns the correct X11 provider.
/// V2 | Window::get_display_server_connection() returns a valid connection pointer.
/// V3 | Window::get_display_server_window() returns a valid window pointer.
fn window_x11_get_display_server() {
    window_x11_prepare!(wx11 {
        // V1 | Window::get_display_server_provider() returns the correct X11 provider.
        assert_eq!(wx11.get_window_provider(), WindowProvider::X11, "Wrong provider given!");
        thread::sleep(WAIT_MS);

        let wx11 = wx11.as_any().downcast_ref::<X11Window>().unwrap();
        
        // V2 | Window::get_display_server_connection() returns a valid connection pointer.
        assert_ne!(wx11.get_display_server_connection(), std::ptr::null_mut(), "Window X11 connection pointer error!");
        thread::sleep(WAIT_MS);

        // V3 | Window::get_display_server_window() returns a valid window pointer.
        assert_ne!(wx11.get_window_handle(), std::ptr::null_mut(), "Window X11 window pointer error!");
        thread::sleep(WAIT_MS);
    });
}

#[test]
#[ignore = "User interaction"]
/// Get X11 Window event count.
/// 
/// # Verification(s)
/// V1 | Window::get_event_count() returns the event count without error.
fn window_x11_get_event_count() {
    window_x11_prepare!(wx11 {

        // V1 | Window::get_event_count() returns the event count without error.
        let _c = wx11.get_event_count();


    });
}


#[test]
#[ignore = "User interaction"]
/// Get and set X11 Window motion mode.
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
fn window_x11_cursor_properties() {
    window_x11_prepare!(wx11 {

        // V1 | Window::get_motion_mode() returns the default motion mode.
        assert_eq!(wx11.get_pointer_mode(), PointerMode::Pointer, "Wrong default cursor mode!");

        // V2 | Window::is_cursor_confined() is false by default.
        assert_eq!(wx11.is_cursor_confined(), false, "Cursor shouldn't be confined by default!");

        // V3 | Window::is_cursor_visible() is true by default.
        assert_eq!(wx11.is_cursor_visible(), true, "Cursor should be visible by default!");

        window_x11_step_loop!("Cursor should be visible and not confined...", wx11, dispatcher, receiver);

        // V4 | Window::hide_cursor() hide cursor without error.
        wx11.hide_pointer();

        window_x11_step_loop!("Cursor should be hidden and not confined...", wx11, dispatcher, receiver);

        // V5 | Calling Window::hide_cursor() again doesn't generate error.
        wx11.hide_pointer();

        // V6 | Window::is_cursor_visible() is false.
        assert_eq!(wx11.is_cursor_visible(), false, "Cursor shouldn't be visible!");

        // V7 | Window::show_cursor() show cursor without error.
        wx11.show_pointer();
        window_x11_step_loop!("Cursor should be visible and not confined...", wx11, dispatcher, receiver);

        // V8 | Calling Window::show_cursor() again doesn't generate error.
        wx11.show_pointer();

        // V9 | Window::is_cursor_visible() is true.
        assert_eq!(wx11.is_cursor_visible(), true, "Cursor should be visible!");

        // V10 | Window::confine_cursor() prevent cursor from exiting boundaries without error.
        wx11.confine_pointer();
        window_x11_step_loop!("Cursor should be visible and confined...", wx11, dispatcher, receiver);

        // V11 | Calling Window::confine_cursor() again doesn't generate error.
        wx11.confine_pointer();

        // V12 | Window::is_cursor_confined() is true.
        assert_eq!(wx11.is_cursor_confined(), true, "Cursor should confined!");

        // V13 | Window::set_cursor_mode() to acceleration keep the cursor in the middle of window.
        wx11.set_pointer_mode(CursorMode::Acceleration);

        // V14 | Window::get_cursor_mode() returns acceleration.
        assert_eq!(wx11.get_cursor_mode(), CursorMode::Acceleration, "Cursor mode should be Acceleration!");
        window_x11_step_loop!("Cursor should be visible, confined and stuck in center...", wx11, dispatcher, receiver);

        // V15 | Window::set_cursor_mode() to pointer release the cursor from the middle of window.
        wx11.set_pointer_mode(CursorMode::Pointer);

        // V16 | Window::get_cursor_mode() returns pointer.
        assert_eq!(wx11.get_cursor_mode(), CursorMode::Pointer, "Cursor mode should be Pointer!");
        window_x11_step_loop!("Cursor should be visible, confined and free to move...", wx11, dispatcher, receiver);

        // V17 | Window::release_cursor() let cursor exit boundaries without error.
        wx11.release_pointer();
        window_x11_step_loop!("Cursor should be visible, released and free to move...", wx11, dispatcher, receiver);

        // V18 | Calling Window::release_cursor() again without error.
        wx11.release_pointer();

        // V19 | Window::is_cursor_confined() is false.
        assert_eq!(wx11.is_cursor_confined(), false, "Cursor shouldn't confined!");

        // V20 | Make cursor hidden. Exiting window must make the cursor reappear and disappear when reentering window.
        wx11.hide_pointer();
        window_x11_step_loop!("Exiting window must make the cursor reappear and disappear when reentering window...", wx11, dispatcher, receiver);

        // V21 | Make cursor confined. Losing focus should release cursor while gaining focus should confine cursor.
        wx11.show_pointer();
        wx11.confine_pointer();
        window_x11_step_loop!("Losing focus should release cursor while gaining focus should confine cursor...", wx11, dispatcher, receiver);

        // V22 | Make cursor mode acceleration. Losing focus should release cursor while gaining focus should confine cursor in center.
        wx11.set_pointer_mode(CursorMode::Acceleration);
        window_x11_step_loop!("Losing focus should release cursor while gaining focus should confine cursor in center...", wx11, dispatcher, receiver);
    });
}



#[test]
#[ignore = "User interaction"]
/// Get and set X11 Window cursor position.
/// 
/// # Verification(s)
/// V1 | Window::get_cursor_position() returns the current cursor position.
/// V2 | Window::set_cursor_position() set the new position without errors.
/// V3 | Window::get_cursor_position() returns the new position.
/// V4 | Change motion mode to Acceleration. Window::set_cursor_position() should give center.
/// V5 | Window::set_cursor_position() set the new position without errors multiple times.
fn window_x11_cursor_position() {
    window_x11_prepare!(wx11 {

        // Confine cursor for test
        wx11.confine_pointer();

        // V1 | Window::get_cursor_position() returns the current cursor position.
        let _cp = wx11.get_cursor_position();
        thread::sleep(WAIT_MS);

        // V2 | Window::set_cursor_position() set the new position without errors.
        wx11.set_pointer_position((CURSOR_X / 2, CURSOR_Y / 2));
        thread::sleep(WAIT_MS);

        // V3 | Window::get_cursor_position() returns the new position.
        let _cp = wx11.get_cursor_position();
        assert_eq!(_cp.0, CURSOR_X / 2, "Cursor X expect {} and not {}!", CURSOR_X / 2, _cp.0);
        assert_eq!(_cp.1, CURSOR_Y / 2, "Cursor Y expect {} and not {}!", CURSOR_Y / 2, _cp.1);
        thread::sleep(WAIT_MS);

        // V4 | Change motion mode to Acceleration. Window::set_cursor_position() should give center.
        wx11.set_pointer_mode(CursorMode::Acceleration);

        let _cp = wx11.get_cursor_position();
        assert_eq!(_cp.0, CURSOR_X, "Cursor X expect {} and not {}!", CURSOR_X, _cp.0);
        assert_eq!(_cp.1, CURSOR_Y, "Cursor Y expect {} and not {}!", CURSOR_Y, _cp.1);
        thread::sleep(WAIT_MS);

        // V5 | Window::set_cursor_position() set the new position without errors multiple times.
        wx11.set_pointer_mode(CursorMode::Pointer);
        for i in 0..255 {
            wx11.set_pointer_position((i * 2,i ));
            
            let _cp = wx11.get_cursor_position();
            assert_eq!(_cp.0, i * 2, "Cursor X expect {} and not {}!", i * 2, _cp.0);
            assert_eq!(_cp.1, i , "Cursor Y expect {} and not {}!", i , _cp.1);

            wx11.dispatch_events(&mut dispatcher, true);

            thread::sleep(WAIT_MS);
        }


    });
}


#[test]
#[ignore = "User interaction"]
/// Get and set X11 Window position.
/// 
/// # Verification(s)
/// V1 | Window::get_position() gives default position.
/// V2 | Window::set_position() work without error.
/// V3 | Window::get_position() return new position.
/// V4 | Window::set_position() multiple time work without error.
fn window_x11_position() {
    window_x11_prepare!(wx11{
        // V1 | Window::get_position() gives default position.
        let pos = wx11.get_position();
        assert!(pos.0 == wx11.get_position().0, "Default Position X error!");
        assert!(pos.1 == wx11.get_position().1, "Default Position Y error!");
        thread::sleep(WAIT_MS);

        // V2 | Window::set_position() work without error.
        wx11.set_position((WINDOW_POS_X,WINDOW_POS_Y));
        thread::sleep(WAIT_MS);

        // V3 | Window::get_position() return new position.
        let pos = wx11.get_position();
        assert!(pos.0 == WINDOW_POS_X, "New Position X error!");
        assert!(pos.1 == WINDOW_POS_Y, "New Position Y error!");
        thread::sleep(WAIT_MS);

        // V4 | Window::set_position() multiple time work without error.
        for i in 0..255 {
            wx11.set_position((i * 5,i * 2));
            

            let pos = wx11.get_position();
            assert!(pos.0 == i * 5, "New Position X error!");
            assert!(pos.1 == i * 2, "New Position Y error!");

            wx11.dispatch_events(&mut dispatcher, true);

            thread::sleep(WAIT_MS);

            
        }
    });
}

#[test]
#[ignore = "User interaction"]
/// Get and set X11 Window size.
/// 
/// # Verification(s)
/// V1 | Window::get_size() returns the default size.
/// V2 | Window::set_size() width < WINDOW_MIN_WIDTH should gives WindowError::WindowSizeError.
/// V3 | Window::set_size() width > WINDOW_MAX_WIDTH should gives WindowError::WindowSizeError.
/// V4 | Window::set_size() height < WINDOW_MIN_HEIGHT should gives WindowError::WindowSizeError.
/// V5 | Window::set_size() height > WINDOW_MAX_HEIGHT should gives WindowError::WindowSizeError.
/// V6 | Window::set_size() work without error when within minimum boundaries.
/// V7 | Window::get_size() return new size.
/// V8 | Window::set_size() work without error when within maximum boundaries.
/// V9 | Window::get_size() return new size.
/// V10 | Window::set_size() multiple time without error.
fn window_x11_size() {
    window_x11_prepare!(wx11 {
        // V1 | Window::get_size() returns the default size.
        let size = wx11.get_size();
        assert_eq!(size.0, WINDOW_WIDTH, "Width expect {} and not {}!", WINDOW_WIDTH, size.0);
        assert_eq!(size.1, WINDOW_HEIGHT, "Height expect {} and not {}!", WINDOW_HEIGHT, size.1);

        // V2 | Window::set_size() width < WINDOW_MIN_WIDTH should gives WindowError::WindowSizeError.
        assert_err!(wx11.set_size((WINDOW_MIN_WIDTH - 1, WINDOW_HEIGHT)), StudioError::Display(DisplayError::SizeError));

        // V3 | Window::set_size() width > WINDOW_MAX_WIDTH should gives WindowError::WindowSizeError.
        assert_err!(wx11.set_size((WINDOW_MAX_WIDTH + 1, WINDOW_HEIGHT)), StudioError::Display(DisplayError::SizeError));
        
        // V4 | Window::set_size() height < WINDOW_MIN_HEIGHT should gives WindowError::WindowSizeError.
        assert_err!(wx11.set_size((WINDOW_WIDTH, WINDOW_MIN_HEIGHT - 1)), StudioError::Display(DisplayError::SizeError));

        // V5 | Window::set_size() height > WINDOW_MAX_HEIGHT should gives WindowError::WindowSizeError.
        assert_err!(wx11.set_size((WINDOW_WIDTH, WINDOW_MAX_HEIGHT + 1)), StudioError::Display(DisplayError::SizeError));

        // V6 | Window::set_size() work without error when within minimum boundaries.
        assert_ok!(wx11.set_size((WINDOW_MIN_WIDTH, WINDOW_MIN_HEIGHT)));

        // V7 | Window::set_size() return new size.
        let size = wx11.get_size();
        assert_eq!(size.0, WINDOW_MIN_WIDTH, "Width expect {} and not {}!", WINDOW_MIN_WIDTH, size.0);
        assert_eq!(size.1, WINDOW_MIN_HEIGHT, "Height expect {} and not {}!", WINDOW_MIN_HEIGHT, size.1);

        // V8 | Window::set_size() work without error when within maximum boundaries.
        assert_ok!(wx11.set_size((WINDOW_MAX_WIDTH, WINDOW_MAX_HEIGHT)));
        
        // V9 | Window::set_size() return new size.
        let size = wx11.get_size();
        assert_eq!(size.0, WINDOW_MAX_WIDTH, "Width expect {} and not {}!", WINDOW_MAX_WIDTH, size.0);
        assert_eq!(size.1, WINDOW_MAX_HEIGHT, "Height expect {} and not {}!", WINDOW_MAX_HEIGHT, size.1);

        // V10 | Window::set_size() multiple time without error.
        for i in 0..255 {
            assert_ok!(wx11.set_size((WINDOW_MIN_WIDTH + i,WINDOW_MIN_HEIGHT + i)));
            

            let size = wx11.get_size();
            assert_eq!(size.0, WINDOW_MIN_WIDTH + i, "Width expect {} and not {}!", WINDOW_MIN_WIDTH + i, size.0);
            assert_eq!(size.1, WINDOW_MIN_HEIGHT + i, "Height expect {} and not {}!", WINDOW_MIN_HEIGHT + i, size.1);

            wx11.dispatch_events(&mut dispatcher, true);

            thread::sleep(WAIT_MS);

            
        }

    });
}

#[test]
#[ignore = "User interaction"]
/// Get and set X11 Window title.
/// 
/// # Verification(s)
/// V1 | Window::get_title() returns the default title.
/// V2 | Window::set_title() set the new title without errors.
/// V3 | Window::get_title() returns the new title.
/// V4 | Window::set_title() multiple time without error.
fn window_x11_title() {
    window_x11_prepare!(wx11 {
        // V1 | Window::get_title() returns the default title.
        assert_eq!(wx11.get_title(), "", "Default title error!");

        // V2 | Window::set_title() set the new title without errors.
        wx11.set_title(WINDOW_TITLE);

        // V3 | Window::get_title() returns the new title.
        assert_eq!(wx11.get_title(), WINDOW_TITLE, "Title expect {:?} and not {:?}!", WINDOW_TITLE, wx11.get_title());

        // V4 | Window::set_title() multiple time without error.
        for i in 0..255 {
            let title = format!("{}{}", "Title", i);
            wx11.set_title(title.as_str());
        }
    });
}

#[test]
#[ignore = "User interaction"]
/// Fullscreen and restore X11 Window test.
/// 
/// # Verification(s)
/// V1 | Window::is_fullscreen(), is_maximized(), is_minimized() all returns false as default.
/// V2 | Window::set_fullscreen() work without error and window now fullscreen.
/// V3 | Window::is_fullscreen() = true, is_maximized() = false, is_minimized() = false.
/// V4 | Window::restore() work without error and window now restored.
/// V5 | Window::is_fullscreen() = false, is_maximized() = false, is_minimized() = false.
/// V6 | Window::set_fullscreen() called multiple time without error.
/// V7 | Window::restore() called multiple time without error.
/// V8 | Multiple chain call of set_fullscreen, restore without error.
fn window_x11_fullscreen_restore() {
    window_x11_prepare!(wx11 {

        wx11.set_title("Default");
        
        // V1 | Window::is_fullscreen(), is_maximized(), is_minimized() all returns false as default.
        assert!(!wx11.is_fullscreen() && !wx11.is_maximized() && !wx11.is_minimized(), "is_fullscreen(), is_maximized(),is_minimized should all be false!");

        window_x11_step_loop!("Window is now at default. Press SPACE to set full screen.", wx11, dispatcher, receiver);

        // V2 | Window::set_fullscreen() work without error and window now fullscreen.
        wx11.set_fullscreen(FullscreenMode::CurrentScreen);

        wx11.set_title("Fullscreen");
        
        window_x11_step_loop!("Window should now be fullscreen. Press SPACE to restore.", wx11, dispatcher, receiver);

        
        // V3 | Window::is_fullscreen() = true, is_maximized() = false, is_minimized() = false.
        assert!(wx11.is_fullscreen() && !wx11.is_maximized() && !wx11.is_minimized(), "Only is_fullscreen() should be true!");
 
        // V4 | Window::restore() work without error and window now restored.
        wx11.restore();

        wx11.set_title("Restored");


        window_x11_step_loop!("Window should now be restored. Press SPACE to continue.", wx11, dispatcher, receiver);

        /*
        // V5 | Window::is_fullscreen() = false, is_maximized() = false, is_minimized() = false.
        assert!(!wx11.is_fullscreen() && !wx11.is_maximized() && !wx11.is_minimized(), "is_fullscreen(), is_maximized(),is_minimized should all be false!");
        window_x11_step_loop!("Window should now be restored. Press SPACE for stress.", wx11, dispatcher, receiver);

        // V6 | Window::set_fullscreen() called multiple time without error.
        for _ in 0..100 {
            wx11.set_fullscreen();
            wx11.sync_events();
        }

        // V7 | Window::restore() called multiple time without error.
        for _ in 0..100 {
            wx11.restore();
            wx11.sync_events();
        }

        // V8 | Multiple chain call of set_fullscreen, restore without error.
        for i in 0..255 {
            if i % 5 == 0 {
                wx11.set_fullscreen();
                wx11.sync_events();
            }
            if i % 7 == 0 {
                wx11.restore();
                wx11.sync_events();
            }
        }
        */
    });
}


#[test]
#[ignore = "User interaction"]
/// Window Close button handle.
/// 
/// # Verification(s)
/// V1 | Window close button is handled without crash.
fn window_x11_close() {

    window_x11_prepare!(wx11, dispatcher, receiver, {
        // V1 | Window close button is handled without crash.


        window_x11_step_loop!("Click the Window X (close) button.", wx11, dispatcher, receiver);
    });

}
