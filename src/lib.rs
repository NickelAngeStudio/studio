// Enable experimental features for documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

// TODO:Develop more
//! Studio is a scalable user interface framework that run native to targeted platform. 

/// Contains enumeration of possible Studio errors.
pub mod error;

/// Window creation and display abstractions.
pub mod display;