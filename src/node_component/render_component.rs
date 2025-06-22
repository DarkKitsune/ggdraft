use ggmath::prelude::*;
use multiverse_ecs::prelude::*;

use crate::app::app_prelude::*;

pub struct RenderComponent {
    /// Called with the parent node upon rendering.
    pub render: fn(&Node, &TargetBuffer, Vector2<u32>, &RenderCamera, &mut GfxCache),
}

impl RenderComponent {
    /// Create a new render component with the given render function.
    pub fn new(
        render: fn(&Node, &TargetBuffer, Vector2<u32>, &RenderCamera, &mut GfxCache),
    ) -> Self {
        Self { render }
    }

    /// Render the node using the render function.
    /// If `render_children` is true, and `universe` is not `None`,
    /// it will also render the children of the node.
    pub fn render(
        &self,
        node: &Node,
        target_buffer: &TargetBuffer,
        buffer_size: Vector2<u32>,
        camera: &RenderCamera,
        render_children: bool,
        cache: &mut GfxCache,
        universe: Option<&Universe>,
    ) {
        // If `render_children` is true and `universe` is provided, render the children.
        if render_children {
            if let Some(universe) = universe {
                for (child, render_component) in universe
                    .nodes_with_handles(node.children())
                    .flatten()
                    .with_component::<RenderComponent>()
                {
                    // Call the render function for each child node with the RenderComponent.
                    render_component.render(
                        child,
                        target_buffer,
                        buffer_size,
                        camera,
                        true,
                        cache,
                        Some(universe),
                    );
                }
            }
        }

        // Call the render function with the node as the argument.
        (self.render)(node, target_buffer, buffer_size, camera, cache);
    }
}
