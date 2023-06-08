use cfg_boost::{target_cfg};

use crate::error::StudioError;

/// Enumeration of possible display errors.
pub mod error;


target_cfg! {
    desktop => {       
        // Desktop display components.
        pub mod desktop;
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