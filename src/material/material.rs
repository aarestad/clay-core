use crate::{material::Colored, prelude::*};
use nalgebra::Vector3;

/// Material of an object surface.
///
/// It specifies the way how does ray bounce off the surface.
/// It defines the color, specularity, opacity, diffusion,
/// radiance and other properties of the object surface.
pub trait Material: Pack + Instance<MaterialClass> {
    /// Brightness of the material.
    ///
    /// If the material emits some light,
    /// the brightnes is equal to maximal color component
    /// in the light emitted, otherwise it is zero.
    fn brightness(&self) -> f64;

    /// Applies color filter to the material
    fn color_with(self, color: Vector3<f64>) -> Colored<Self> {
        Colored::new(self, color)
    }
}

/// Device interface for material.
///
/// How to implement in OpenCL:
/// ```c
/// #include <clay_core/material/material.h>
///
/// MATERIAL_BOUNCE_RET <material>_bounce(
///     MATERIAL_BOUNCE_ARGS_DEF
/// ) {
///     ...
/// }
/// ```
pub enum MaterialClass {}
impl Class for MaterialClass {
    fn name() -> String {
        "material".to_string()
    }
    fn methods() -> Vec<String> {
        vec!["bounce".to_string()]
    }
}
