use studio::display::desktop::{Window, property::{WindowProperty, WindowEventWaitMode, WindowPositionOption, DEFAULT_WIDTH, DEFAULT_HEIGHT, KeyboardMode, PointerMode}};





/// Changes window properties and verify if changes are made.
/// 
/// # Verification(s)
/// V1 | Verify that Parent(&'window Window<'window>, SubWindowOption) changes WindowProperty.parent.
/// V2 | Verify that RemoveParent changes WindowProperty.parent
/// V3 | Verify that EventWaitMode(NeverWait) changes WindowProperty.wait_mode to NeverWait
/// V4 | Verify that EventWaitMode(AlwaysWait) changes WindowProperty.wait_mode to AlwaysWait
/// V5 | Verify that Title(String) changes WindowProperty.title
/// V6 | Verify that Position(Desktop((i32, i32))) changes WindowProperty.relative_position to Desktop((i32, i32)).
/// V7 | Verify that Position(Screen(Screen, (i32, i32))) changes WindowProperty.relative_position to Screen(Screen, (i32, i32)).
/// V8 | Verify that Position(Parent((i32, i32))) changes WindowProperty.relative_position to Parent((i32, i32)).
/// V9 | Verify that Position(CenterScreen(Screen)) changes WindowProperty.relative_position to CenterScreen(Screen).
/// V10 | Verify that Position(CenterParent) changes WindowProperty.relative_position to CenterParent.
/// V11 | Verify that Size((u32, u32)) changes WindowProperty.size and center
/// V12 | Verify that HideDecoration changes WindowProperty.decoration to false
/// V13 | Verify that ShowDecoration changes WindowProperty.decoration to true
/// V14 | Verify that Minimize changes WindowProperty.minimized
/// V15 | Verify that Maximized changes WindowProperty.maximized
/// V16 | Verify that FullScreen(Current) changes WindowProperty.fullscreen to FullScreenMode::Current.
/// V17 | Verify that FullScreen(Primary) changes WindowProperty.fullscreen to FullScreenMode::Primary.
/// V18 | Verify that FullScreen(Desktop) changes WindowProperty.fullscreen to FullScreenMode::Desktop.
/// V19 | Verify that FullScreen(Screen(Screen)) changes WindowProperty.fullscreen to FullScreenMode::Screen(Screen).
/// V20 | Verify that Restore changes WindowProperty.minimized to false
/// V21 | Verify that Restore changes WindowProperty.maximized to false
/// V22 | Verify that Restore changes WindowProperty.fullscreen to None
/// V23 | Verify that KeyboardPropertySet::SetMode(DirectInput) changes KeyboardProperty.mode to DirectInput
/// V24 | Verify that KeyboardPropertySet::SetMode(TextInput) changes KeyboardProperty.mode to TextInput
/// V25 | Verify that KeyboardPropertySet::EnableAutoRepeat changes KeyboardProperty.auto_repeat to true
/// V26 | Verify that KeyboardPropertySet::DisableAutoRepeat changes KeyboardProperty.auto_repeat to false
/// V27 | Verify that PointerPropertySet::PointerMode(cursor) changes PointerProperty.mode to Cursor
/// V28 | Verify that PointerPropertySet::PointerMode(Acceleration) changes PointerProperty.mode to Acceleration
/// V29 | Verify that PointerPropertySet::Position((i32, i32)) changes PointerProperty.position to (i32,i32)
/// V30 | Verify that PointerPropertySet::Hide changes PointerProperty.visible to false
/// V31 | Verify that PointerPropertySet::Show changes PointerProperty.visible to true
/// V32 | Verify that PointerPropertySet::Confine changes PointerProperty.confined to true
/// V33 | Verify that PointerPropertySet::Release changes PointerProperty.confined to false
#[test]
fn test_window_properties_set(){

   

}