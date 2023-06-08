use cfg_boost::target_cfg;

// Misc ressources for tests
pub mod rsrcs;

// Window events tests
mod events;



/*********
* CONSTS *
*********/
/// Window dimension
pub const WINDOW_WIDTH:u32 = 640;
pub const WINDOW_HEIGHT:u32 = 480;

target_cfg! {
    linux => {
        #[test]
        #[ignore = "User interaction needed"]
        fn x11_window_tests() {
            let mut window = crate::assert_ok!(studio::display::desktop::provider::linux::get_x11_window(WINDOW_WIDTH, WINDOW_HEIGHT));

            rsrcs::print_instructions_header();

            events::window_events_tests(&mut window);

            rsrcs::print_instructions_footer();

            

        }
    }
}
