//! Lib used as support for cfd-rs and cfd-rs-mesh.
//! Mostly for type definition and common functions.
//!
//! For now only functionnalities to construct an advancing front algorithm are being created.

pub use mesh::*;
pub use indices::*;

pub mod mesh;
pub mod indices;
