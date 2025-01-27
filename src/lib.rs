//! Core functionality for [Clay project](https://clay-rs.github.io/).

/// Own error type.
pub mod error;
/// Own result type.
pub mod result;

/// Serialization of entities for storing them on the device.
pub mod pack;
/// Pushing arguments to the device kernel.
pub mod push;
/// Representation of entities that could be stored in the device.
pub mod store;
/// Rust type hashing to generate unique identfiers in device code.
pub mod type_hash;

/// Traits for device code interfaces definition.
pub mod class;
/// Basic macro for making a union of entities.
pub mod select;

/// Mappings in render space.
pub mod map;
/// Material of an object.
pub mod material;
/// Object to render.
pub mod object;
/// Shape of an object.
pub mod shape;

/// Scene to be rendered.
pub mod scene;
/// View of the scene.
pub mod view;

/// Filter for rendered image postprocessing.
pub mod filter;

/// Various device buffers.
pub mod buffer;
/// Context of the device code execution.
pub mod context;
/// Functionality for rendering pipeline.
pub mod process;

/// Loading the device OpenCL source code.
pub mod source;

/// Reexport of the basic traits.
pub mod prelude {
    pub use crate::class::*;
    pub use crate::pack::*;
    pub use crate::push::*;
    pub use crate::store::*;
    pub use crate::type_hash::*;
}

pub use error::Error;
pub use result::Result;

pub use context::*;
pub use prelude::*;
pub use source::*;
