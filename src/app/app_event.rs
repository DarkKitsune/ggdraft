use game::{chunk::CHUNK_SIZE, world::World, world_generator::WorldGenerator};

use crate::{gfx::render_camera::RenderCamera, svector};

use super::app_prelude::*;

// Called when the app is initialized
pub fn init(
    _engine: &mut Engine,
    _universe: &mut Universe,
    _async_data: AppData<AsyncData>,
) -> AppEventResult<()> {
    println!("App has been initialized.");
    Ok(())
}

// Called when the window receives events
pub fn window_events(
    engine: &mut Engine,
    _universe: &mut Universe,
    _async_data: AppData<AsyncData>,
    _graphics_cache: &mut GfxCache,
    window_events: &WindowEvents,
) -> AppEventResult<()> {
    // Stop the engine if the window is closed or the Escape key is pressed
    if window_events.closed() || window_events.key_pressed(Key::Escape) {
        engine.stop();
    }

    Ok(())
}

// Called before the engine thinks
pub fn pre_think(
    _engine: &mut Engine,
    _universe: &mut Universe,
    _async_data: AppData<AsyncData>,
) -> AppEventResult<()> {
    Ok(())
}

// Called when the engine thinks
pub fn think(
    _engine: &mut Engine,
    _universe: &mut Universe,
    _async_data: AppData<AsyncData>,
) -> AppEventResult<()> {
    Ok(())
}

// Called when initializing the rendering engine
pub fn init_render(
    _engine: &mut Engine,
    _universe: &mut Universe,
    _async_data: AppData<AsyncData>,
    graphics_cache: &mut GfxCache,
) -> AppEventResult<()> {
    // Create vertex layout describing the vertices going into the shader
    let vertex_layout = graphics_cache.create_vertex_layout(Some("vertex layout"), |layout| {
        layout.with_position().with_color().with_normal()
    });

    // Create an input layout from the vertex layout
    graphics_cache.create_input_layout_from_vertex_layout(Some("input layout"), &vertex_layout);

    // Create shader program
    graphics_cache.create_program_vertex_fragment(
        // Name of the program
        Some("program"),
        // Name of the input layout to use
        "input layout",
        // Vertex shader
        |inputs, parameters, outputs| {
            // Get the vertex inputs
            let position = inputs.get(VertexInput::Position)?;
            let color = inputs.get(VertexInput::Color)?;
            let normal = inputs.get(VertexInput::Normal)?;

            // Get the view, projection, and model matrices
            let view_matrix = parameters.get_view_matrix();
            let projection_matrix = parameters.get_projection_matrix();

            // Convert the input position to screen space
            let screen_space = projection_matrix * view_matrix * position.append(1.0);

            // Output the screen space position
            outputs.set_vertex_position(screen_space);

            // Set the color to the input color, or white if no color is provided
            outputs.set("color", color)?;

            // Set the normal to the input normal
            outputs.set("normal", normal)?;

            Ok(())
        },
        // Fragment shader
        |inputs, _parameters, outputs| {
            // Get the fragment inputs
            let input_color = inputs.get("color")?;
            let input_normal = inputs.get("normal")?;

            // Calculate basic lighting
            let brightness = input_normal
                .dot(vector!(1.0, 4.0, -2.0).normalized())
                .max(0.0);

            // Fragment color = input color * brightness
            let output_color =
                input_color * svector!(brightness.clone(), brightness.clone(), brightness, 1.0);
            outputs.set_fragment_color(output_color);

            Ok(())
        },
    )?;

    // Create a test world
    let mut world = World::new(WorldGenerator::new(12345));

    // Create the meshes
    graphics_cache.create_mesh(
        Some("mesh0"),
        &vertex_layout,
        world.ensure_chunk(vector!(0, 0, 0)),
    );
    graphics_cache.create_mesh(
        Some("mesh1"),
        &vertex_layout,
        world.ensure_chunk(vector!(-1, 0, 0)),
    );
    graphics_cache.create_mesh(
        Some("mesh2"),
        &vertex_layout,
        world.ensure_chunk(vector!(1, 0, 0)),
    );
    graphics_cache.create_mesh(
        Some("mesh3"),
        &vertex_layout,
        world.ensure_chunk(vector!(0, 0, -1)),
    );
    graphics_cache.create_mesh(
        Some("mesh4"),
        &vertex_layout,
        world.ensure_chunk(vector!(0, 0, 1)),
    );


    println!("Render initialized.");

    Ok(())
}

// Called when the engine renders a frame
pub fn render(
    _engine: &mut Engine,
    _universe: &mut Universe,
    _async_data: AppData<AsyncData>,
    graphics_cache: &mut GfxCache,
    framebuffer: TargetBuffer,
    framebuffer_size: Vector2<u32>,
) -> AppEventResult<()> {
    // Clear the framebuffer with a color
    framebuffer.clear_with_color(
        BLUE // Start with blue
            .lerp(&GRAY, 0.5) // Mix 50% with gray
            .lerp(&WHITE, 0.25), // Mix 25% with white
    );

    // Clear the framebuffer depth
    framebuffer.clear_depth();

    // Retrieve the program and input layout
    let program = graphics_cache.get("program").unwrap();
    let input_layout = graphics_cache.get("input layout").unwrap();

    // Retrieve the mesh
    let mesh0 = graphics_cache.get("mesh0").unwrap();
    let mesh1 = graphics_cache.get("mesh1").unwrap();
    let mesh2 = graphics_cache.get("mesh2").unwrap();
    let mesh3 = graphics_cache.get("mesh3").unwrap();
    let mesh4 = graphics_cache.get("mesh4").unwrap();

    // Begin the parameters for rendering
    let mut parameters = RenderParameters::new();

    // Set the camera matrices
    let target = vector!(CHUNK_SIZE as f32 / 2.0, 0.0, CHUNK_SIZE as f32 / 2.0);
    let rotation = Quaternion::from_rotation_x(-0.5).and_then(&Quaternion::from_rotation_y(-0.7));
    let camera = RenderCamera::perspective_looking_at(target, rotation, 40.0, 75.0, 0.01, 100.0);
    parameters.set_camera(framebuffer_size.convert_to().unwrap(), &camera);

    // Draw the triangle
    framebuffer.render_mesh(program, input_layout, &parameters, &mesh0)?;
    framebuffer.render_mesh(program, input_layout, &parameters, &mesh1)?;
    framebuffer.render_mesh(program, input_layout, &parameters, &mesh2)?;
    framebuffer.render_mesh(program, input_layout, &parameters, &mesh3)?;
    framebuffer.render_mesh(program, input_layout, &parameters, &mesh4)?;

    Ok(())
}
