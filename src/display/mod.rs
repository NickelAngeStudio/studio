use cfg_boost::{target_cfg};

target_cfg! {
    desktop => {       
        // Desktop display components.
        pub mod desktop;

        // Reimport desktop error as DisplayError.
        pub use desktop::error::DesktopError as DisplayError;
    },

    mobile => {
        /// Mobile display components.
        pub mod mobile;
    },

    wasm => {
        /// Web assembly display components
        pub mod wasm;
    }
}