//! Lib used as support for cfd-rs and cfd-rs-mesh.
//! Mostly for type definition and common functions.
//!
//! For now only functionnalities to construct an advancing front algorithm are being created.

pub use error::*;
pub use indices::*;
pub use mesh::*;

pub mod error;
pub mod indices;
pub mod mesh;
