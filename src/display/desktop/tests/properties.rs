use crate::display::desktop::property::{WindowProperty, WindowPositionOption, DEFAULT_WIDTH, DEFAULT_HEIGHT, KeyboardMode, PointerMode};

/// Shortcut macro to test property.
macro_rules! test_property {
    ($properties : expr, $member : ident, $expect : expr) => {
        assert_eq!($properties.$member, $expect, "WindowProperty.{} should be {}!", stringify!($member), stringify!($expect));
    };

    ($properties : expr, $member : ident . $funct : ident (), $expect : expr) => {
        assert_eq!($properties.$member.$funct(), $expect, "WindowProperty.{} should be {}!", stringify!($member.$funct()), stringify!($expect));
    };

    ($properties : expr, $member : ident . $submember : ident, $expect : expr) => {
        assert_eq!($properties.$member.$submember, $expect, "WindowProperty.{} should be {}!", stringify!($member.$submember), stringify!($expect));
    };
}

/// Test a window default properties
/// 
/// # Verification(s)
/// V1 | Verify that window has no show option as default.
/// V2 | Verify that [WindowEventWaitMode]  is set to [WindowEventWaitMode::NeverWait].
/// V3 | Verify that title is an empty string.
/// V4 | Verify that position is set to (0,0)
/// V5 | Verify that relative position is set to WindowPositionOption::Desktop((0,0)).
/// V6 | Verify that size is set to (DEFAULT_WIDTH, DEFAULT_HEIGHT).
/// V7 | Verify that centeris set to (DEFAULT_WIDTH as i32 / 2, DEFAULT_HEIGHT as i32 / 2), 
/// V8 | Verify that decoration is set to true.
/// V9 | Verify that minimized is set to false.
/// V10 | Verify that maximized is set to false.
/// V11 | Verify that fullscreen is set to Option::None.
/// V12 | Verify that visible is false.
/// V13 | Verify that created is false.
/// V15 | Verify that [KeyboardMode] is set to [KeyboardMode::DirectInput].
/// V16 | Verify that auto repeat is set to false.
/// V17 | Verify that [PointerMode] is set to [PointerMode::Cursor]
/// V18 | Verify that pointer position is set to (0,0)
/// V19 | Verify that pointer visible is true.
/// V20 | Verify that pointer confined is false.
pub fn test_window_default_properties(properties : &WindowProperty) {

    // V1 | Verify that window has no show options as default.
    test_property!(properties, show_option.is_none(), true);

    // V3 | Verify that title is an empty string.
    test_property!(properties, title, "");

    // V4 | Verify that position is set to (0,0)
    test_property!(properties, position, (0,0));

    // V5 | Verify that relative position is set to WindowPositionOption::Desktop((0,0)).
    test_property!(properties, relative_position, WindowPositionOption::Desktop((0,0)));

    // V6 | Verify that size is set to (DEFAULT_WIDTH, DEFAULT_HEIGHT).
    test_property!(properties, size, (DEFAULT_WIDTH, DEFAULT_HEIGHT));

    // V7 | Verify that center is set to (DEFAULT_WIDTH as i32 / 2, DEFAULT_HEIGHT as i32 / 2), 
    test_property!(properties, center, (DEFAULT_WIDTH as i32 / 2, DEFAULT_HEIGHT as i32 / 2));

    // V8 | Verify that decoration is set to true.
    test_property!(properties, decoration, true);

    // V9 | Verify that minimized is set to false.
    test_property!(properties, minimized, false);

    // V10 | Verify that maximized is set to false.
    test_property!(properties, maximized, false);

    // V11 | Verify that fullscreen is set to Option::None.
    test_property!(properties, fullscreen, Option::None);

    // V12 | Verify that visible is false.
    test_property!(properties, visible, false);

    // V13 | Verify that created is false.
    test_property!(properties, created, false);

    // V15 | Verify that [KeyboardMode] is set to [KeyboardMode::DirectInput].
    test_property!(properties, keyboard.mode, KeyboardMode::DirectInput);

    // V16 | Verify that auto repeat is set to false.
    test_property!(properties, keyboard.auto_repeat, false);

    // V17 | Verify that [PointerMode] is set to [PointerMode::Cursor]
    test_property!(properties, pointer.mode, PointerMode::Cursor);

    // V18 | Verify that pointer position is set to (0,0)
    test_property!(properties, pointer.position, (0,0));

    // V19 | Verify that pointer visible is true.
    test_property!(properties, pointer.visible, true);

    // V20 | Verify that pointer confined is false.
    test_property!(properties, pointer.confined, false);

}