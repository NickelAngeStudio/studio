use cfg_boost::{target_cfg};

use crate::error::StudioError;

/// Enumeration of possible display errors.
pub mod error;



target_cfg! {
    desktop => {       
        // Desktop components abstraction of display.
        pub mod desktop;

        #[doc(inline)]
        pub use desktop::create_window;

        
    },

    mobile => {
        /// Mobile components of display.
        pub mod mobile;
    }
}