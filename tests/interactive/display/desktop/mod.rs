use cfg_boost::target_cfg;

// Misc ressources for tests
pub mod rsrcs;

// Window properties tests
mod properties;

// Window events tests
mod events;

// Window that print all events to console.
mod log;


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
            
            loop {
                match rsrcs::select_options(){
                    rsrcs::InputSelection::Events => events::window_events_tests(),
                    rsrcs::InputSelection::Properties => properties::window_properties_tests(),
                    rsrcs::InputSelection::Methods => todo!(),
                    rsrcs::InputSelection::LogWindow => log::log_window(),
                    rsrcs::InputSelection::Quit => break,
                }
            }

            rsrcs::print_instructions_footer();

            

        }
    }
}
