use cfg_boost::target_cfg;

target_cfg! {
    linux => {
        /// X11 Window Manager integration tests
        pub mod x11;
    }
}