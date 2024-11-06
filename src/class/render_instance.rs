use multiverse_ecs::prelude::*;

use crate::{app::app_prelude::RenderParameters, geometry::orientation::Orientation};

define_class! {
    /// Represents an instance for rendering.
    /// Instances should be added as children to a renderer.
    pub class RenderInstance {
        /// The instance's orientation.
        orientation: Orientation,
        /// The instance's render parameters.
        render_parameters: RenderParameters,
    }
}
