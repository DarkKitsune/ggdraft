use multiverse_ecs::prelude::*;

use crate::{app::app_prelude::RenderParameters, geometry::orientation::Orientation};

define_class!{
    /// Renders a mesh.
    /// Add nodes with the `RenderInstance` class as children to the renderer.
    pub class MeshRenderer {
        /// The base orientation.
        orientation: Orientation,
        /// The base render parameters.
        /// Instances can override these parameters with their own.
        render_parameters: RenderParameters,
    }
}