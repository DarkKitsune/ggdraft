use ggmath::prelude::*;
use multiverse_ecs::prelude::*;

use crate::{app::app_prelude::{RenderParameters, TargetBuffer}, geometry::orientation::Orientation, gfx::{gfx_cache::{CacheHandle, GfxCache}, render_camera::RenderCamera}, node_component::render_component::RenderComponent};

define_class! {
    /// Renders a mesh.
    /// Add nodes with the `RenderInstance` class as children to the renderer.
    pub class MeshRenderer {
        /// The base orientation.
        orientation: Orientation,
        /// The mesh to render.
        mesh: CacheHandle,
        /// The input layout for the mesh.
        input_layout: CacheHandle,
        /// The program to use for rendering the mesh.
        program: CacheHandle,
        /// Parameters passed when rendering the mesh.
        parameters: RenderParameters,
        /// The render component that will render the mesh.
        render_component: RenderComponent
    }
}
impl MeshRenderer {
    /// Create a new MeshRenderer.
    pub fn new(orientation: Orientation, mesh: CacheHandle, input_layout: CacheHandle, program: CacheHandle, parameters: RenderParameters) -> Self {
        // Create a render component that will render the mesh.
        let render_component = RenderComponent::new(Self::__render);

        Self {
            orientation,
            mesh,
            input_layout,
            program,
            parameters,
            render_component,
        }
    }

    /// Supplied to the render component.
    fn __render(node: &Node, target_buffer: &TargetBuffer, buffer_size: Vector2<u32>, camera: &RenderCamera, cache: &mut GfxCache) {
        // Render the mesh using the node's orientation and mesh.
        if let Some(mesh_renderer) = node.class_as::<MeshRenderer>() {
            // Get mesh, input layout, and program from the cache.
            let mesh = cache.get_mesh(&mesh_renderer.mesh).expect("Mesh not found in cache");
            let input_layout = cache.get_input_layout(&mesh_renderer.input_layout).expect("Input layout not found in cache");
            let program = cache.get_program(&mesh_renderer.program).expect("Program not found in cache");

            // Clone the parameters because we need to modify them.
            let mut parameters = mesh_renderer.parameters.clone();
            
            // Set the model matrix and camera matrices in the parameters.
            parameters.set_model_matrix(mesh_renderer.orientation.get_transform());
            parameters.set_camera(buffer_size.convert_to().unwrap(), camera);
            
            target_buffer.render_mesh(program, input_layout, &parameters, mesh).unwrap();
        } else {
            panic!("Node is not a MeshRenderer");
        }
    }
}