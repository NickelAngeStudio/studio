use cfg_boost::target_cfg;

// Misc ressources for tests
pub mod rsrcs;

// Window events tests
mod events;

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
                    rsrcs::InputSelection::Properties => todo!(),
                    rsrcs::InputSelection::Methods => todo!(),
                    rsrcs::InputSelection::LogWindow => log::log_window(),
                    rsrcs::InputSelection::Quit => break,
                }
            }

            rsrcs::print_instructions_footer();

            

        }
    }
}
