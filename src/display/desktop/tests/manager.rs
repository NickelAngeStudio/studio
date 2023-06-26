use cfg_boost::{match_cfg, target_cfg};

use crate::display::desktop::{manager::{WindowManager, WindowProvider}, event::Event, property::{WindowPositionOption, DEFAULT_WIDTH, DEFAULT_HEIGHT}, screen::Screen, window::WindowShowOption};

use super::properties::test_window_default_properties;

/// Screen Size when screen is needed
const SCREEN_SIZE: (u32, u32) = (1920, 1080); 

/// Tests a static [WindowManager] implementation. No user interactions needed  and can
/// be automated. Used to complement a valid interactive test.
/// 
/// Verification(s)
/// V1 | WindowManager::new() create a new instance without error.
/// V2 | WindowManager::get_window_provider() return the provider given as parameter.
/// V3 | WindowManager::get_properties() return a valid default set of properties.
/// V4 | WindowManager::push_event(&self, event: Event) should work without error.
/// V5 | WindowManager::poll_event() should return previously pushed event.
/// V6 | WindowManager::remove_parent() should not panic! with no parent.
/// V7 | WindowManager::set_title() should change the title property.
/// V8 | WindowManager::set_position(Desktop((i32, i32))) should change the absolute and relative properties.
/// V9 | WindowManager::set_position(Screen(Screen, (i32, i32))) should change the absolute and relative properties.
/// V10 | WindowManager::set_position(Parent((i32, i32))) should change the absolute and relative properties.
/// V11 | WindowManager::set_position(CenterScreen(Screen)) should change the absolute and relative properties.
/// V12 | WindowManager::set_position(CenterParent) should change the relative properties.
/// V13 | WindowManager::set_size() should change the window size.
/// V14 | WindowManager::hide_decoration() should change the decoration property to false.
/// V15 | WindowManager::show_decoration() should change the decoration property to true.
/// V16 | WindowManager::minimize() should change WindowProperty.minimized to true.
/// V17 | WindowManager::maximize() should change WindowProperty.mazimized to true.
/// V18 | WindowManager::set_event_wait_mode(AlwaysWait) changes WindowProperty.wait_mode to AlwaysWait
/// V19 | WindowManager::set_event_wait_mode(NeverWait) changes WindowProperty.wait_mode to NeverWait
/// V20 | WindowManager::set_fullscreen(Current) changes WindowProperty.fullscreen to FullScreenMode::Current.
/// V21 | WindowManager::set_fullscreen(Primary) changes WindowProperty.fullscreen to FullScreenMode::Primary.
/// V22 | WindowManager::set_fullscreen(Desktop) changes WindowProperty.fullscreen to FullScreenMode::Desktop.
/// V23 | WindowManager::set_fullscreen(Screen(Screen)) changes WindowProperty.fullscreen to FullScreenMode::Screen(Screen).
/// V24 | WindowManager::Restore() changes WindowProperty.minimized to false
/// V25 | WindowManager::Restore() changes WindowProperty.maximized to false
/// V26 | WindowManager::Restore() changes WindowProperty.fullscreen to None
/// V27 | WindowManager::set_keyboard_mode(TextInput) changes KeyboardProperty.mode to TextInput
/// V28 | WindowManager::set_keyboard_mode(DirectInput) changes KeyboardProperty.mode to DirectInput
/// V29 | WindowManager::set_keyboard_auto_repeat(true) changes KeyboardProperty.auto_repeat to True
/// V30 | WindowManager::set_keyboard_auto_repeat(false) changes KeyboardProperty.auto_repeat to True
/// V31 | WindowManager::set_pointer_mode(Acceleration) changes PointerProperty.mode to Acceleration
/// V32 | WindowManager::set_pointer_mode(cursor) changes PointerProperty.mode to cursor
/// V33 | WindowManager::set_pointer_position((i32, i32)) changes PointerProperty.position to (i32,i32)
/// V34 | WindowManager::hide_pointer changes PointerProperty.visible to false
/// V35 | WindowManager::show_pointer changes PointerProperty.visible to true
/// V36 | WindowManager::confine_pointer changes PointerProperty.confined to true
/// V37 | WindowManager::release_pointer changes PointerProperty.confined to false
/// V38 | WindowManager::is_key_shift_down(0) must return false.
/// V39 | WindowManager::is_key_ctrl_down(0) must return false.
/// V40 | WindowManager::is_key_alt_down(0) must return false.
/// V41 | WindowManager::is_key_meta_down(0) must return false.
/// V42 | WindowManager::is_key_command_down(0) must return false.
/// V43 | WindowManager::is_key_hyper_down(0) must return false.
/// V44 | WindowManager::is_capslock_on(0) must return false.
/// V45 | WindowManager::is_numlock_on(0) must return false.
pub fn test_non_interactive_manager_impl<WM : WindowManager>(provider : WindowProvider){

    match WM::new(){
        // V1 | WindowManager::new() create a new instance without error.
        Ok(mut wm) => {
            // V2 | WindowManager::get_window_provider() return the provider given as parameter.
            assert_eq!(wm.get_window_provider(), provider);

            // V3 | WindowManager::get_properties() return a valid default set of properties.
            test_window_default_properties(wm.get_properties());

            // V4 | WindowManager::push_event(&self, event: Event) should work without error.
            let ev1 = Event::None;
            wm.push_event(ev1.clone());

            // V5 | WindowManager::poll_event() should return previously pushed event.
            let ev2 = wm.poll_event();

            match_cfg! {
                !immediate:ft => {
                    assert_eq!(ev1, ev2);
                }, 
                _ => {
                    assert_eq!(ev1, *ev2);
                }
            }
           

            // V7 | WindowManager::set_title() should change the title property.
            let s = String::from("HELLO!");
            wm.set_title(&s);
            assert_eq!(s, wm.get_properties().title);

            match_cfg! {
                !immediate:ft => {  // Retained mode
                    wm.show(WindowShowOption::Normal, Option::None);
                },
                _ => {  // Immediate mode
                    wm.show();
                }
            }
            

            // V8 | WindowManager::set_position(Desktop((i32, i32))) should change the absolute and relative properties.
            let abs_pos: (i32,i32) = (1,1);
            let rel_pos = WindowPositionOption::Desktop(abs_pos);
            wm.set_position(rel_pos.clone());
            assert_eq!(wm.get_properties().relative_position, rel_pos);
            assert_eq!(wm.get_properties().position, abs_pos);

            // V9 | WindowManager::set_position(Screen(Screen, (i32, i32))) should change the absolute and relative properties.
            let screen = Screen::new(String::from("test"), (10,10), SCREEN_SIZE, 60000, true, Vec::new());
            let abs_pos: (i32,i32) = (11,11);
            let rel_pos = WindowPositionOption::Screen(screen, (1,1));
            wm.set_position(rel_pos.clone());
            assert_eq!(wm.get_properties().relative_position, rel_pos);
            assert_eq!(wm.get_properties().position, abs_pos);

            match_cfg! {
                !immediate:ft => {  // Tests only for retained mode
                    // V10 | WindowManager::set_position(Parent((i32, i32))) should change the absolute and relative properties.
                    let abs_pos: (i32,i32) = (2,2);
                    let rel_pos = WindowPositionOption::Parent(abs_pos);
                    wm.set_position(rel_pos.clone());
                    assert_eq!(wm.get_properties().relative_position, rel_pos);
                    assert_eq!(wm.get_properties().position, abs_pos);

                    // V12 | WindowManager::set_position(CenterParent) should change the relative properties.
                    let rel_pos = WindowPositionOption::CenterParent;
                    wm.set_position(rel_pos.clone());
                    assert_eq!(wm.get_properties().relative_position, rel_pos);
                },
                _ => {}
            }

            // V11 | WindowManager::set_position(CenterScreen(Screen)) should change the absolute and relative properties.
            let screen = Screen::new(String::from("test"), (10,10), SCREEN_SIZE, 60000, true, Vec::new());
            let abs_pos: (i32,i32) = ((10 + (SCREEN_SIZE.0 - DEFAULT_WIDTH) / 2) as i32,(10 + (SCREEN_SIZE.1 - DEFAULT_HEIGHT) / 2) as i32);
            let rel_pos = WindowPositionOption::CenterScreen(screen);
            wm.set_position(rel_pos.clone());
            assert_eq!(wm.get_properties().relative_position, rel_pos);
            assert_eq!(wm.get_properties().position, abs_pos);

            

            // V13 | WindowManager::set_size() should change the window size.


            // Close the window before the next tests
            wm.close();


            // V14 | WindowManager::hide_decoration() should change the decoration property to false.


            // V15 | WindowManager::show_decoration() should change the decoration property to true.


            // V16 | WindowManager::minimize() should change WindowProperty.minimized to true.


            // V17 | WindowManager::maximize() should change WindowProperty.mazimized to true.


            // V18 | WindowManager::set_event_wait_mode(AlwaysWait) changes WindowProperty.wait_mode to AlwaysWait


            // V19 | WindowManager::set_event_wait_mode(NeverWait) changes WindowProperty.wait_mode to NeverWait


            // V20 | WindowManager::set_fullscreen(Current) changes WindowProperty.fullscreen to FullScreenMode::Current.


            // V21 | WindowManager::set_fullscreen(Primary) changes WindowProperty.fullscreen to FullScreenMode::Primary.


            // V22 | WindowManager::set_fullscreen(Desktop) changes WindowProperty.fullscreen to FullScreenMode::Desktop.


            // V23 | WindowManager::set_fullscreen(Screen(Screen)) changes WindowProperty.fullscreen to FullScreenMode::Screen(Screen).


            // V24 | WindowManager::Restore() changes WindowProperty.minimized to false


            // V25 | WindowManager::Restore() changes WindowProperty.maximized to false


            // V26 | WindowManager::Restore() changes WindowProperty.fullscreen to None


            // V27 | WindowManager::set_keyboard_mode(TextInput) changes KeyboardProperty.mode to TextInput


            // V28 | WindowManager::set_keyboard_mode(DirectInput) changes KeyboardProperty.mode to DirectInput


            // V29 | WindowManager::set_keyboard_auto_repeat(true) changes KeyboardProperty.auto_repeat to True


            // V30 | WindowManager::set_keyboard_auto_repeat(false) changes KeyboardProperty.auto_repeat to True


            // V31 | WindowManager::set_pointer_mode(Acceleration) changes PointerProperty.mode to Acceleration


            // V32 | WindowManager::set_pointer_mode(cursor) changes PointerProperty.mode to cursor


            // V33 | WindowManager::set_pointer_position((i32, i32)) changes PointerProperty.position to (i32,i32)


            // V34 | WindowManager::hide_pointer changes PointerProperty.visible to false


            // V35 | WindowManager::show_pointer changes PointerProperty.visible to true


            // V36 | WindowManager::confine_pointer changes PointerProperty.confined to true


            // V37 | WindowManager::release_pointer changes PointerProperty.confined to false


            // V38 | WindowManager::is_key_shift_down(0) must return false.


            // V39 | WindowManager::is_key_ctrl_down(0) must return false.


            // V40 | WindowManager::is_key_alt_down(0) must return false.


            // V41 | WindowManager::is_key_meta_down(0) must return false.


            // V42 | WindowManager::is_key_command_down(0) must return false.


            // V43 | WindowManager::is_key_hyper_down(0) must return false.


            // V44 | WindowManager::is_capslock_on(0) must return false.


            // V45 | WindowManager::is_numlock_on(0) must return false.

        },
        Err(_) => panic!("Error while creating WindowManager!"),
    }

}