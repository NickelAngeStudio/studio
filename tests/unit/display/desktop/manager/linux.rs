use crate::display::desktop::{tests::manager::test_non_interactive_manager_impl, manager::WindowProvider};

use super::{LinuxWindowManager, wayland::WaylandWindowManager};

/// LinuxWindowManager automated unit tests.
///
/// # Verification(s)
/// V1 | Execute the test_non_interactive_manager_impl for LinuxWindowManager.
#[test]
fn unit_test(){
	test_non_interactive_manager_impl::<LinuxWindowManager>(if WaylandWindowManager::is_supported() {
		WindowProvider::Wayland
	} else {
		WindowProvider::X11
	});
}

/// Ignored test description
///
/// # Verification(s)
/// V1 | Description of aspect verified
#[test]
#[ignore = "Must be executed manually"]
fn ignored_test(){
	todo!()

	
	
}