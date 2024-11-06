use std::collections::HashMap;

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

// Called after the engine thinks
pub fn post_think(
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
        layout
            .with_position()
            .with_color()
            .with_normal()
            .with_tex_coord()
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
        |inputs, _parameters, outputs| {
            // Set the vertex position to the input position
            outputs.set_vertex_position(inputs.get(VertexInput::Position)?.append(1.0));

            // Set the color to the input color, or white if no color is provided
            outputs.set(
                "color",
                inputs.get(VertexInput::Color).unwrap_or(WHITE.into()),
            )?;

            // Set the texture coordinates to the input texture coordinates
            outputs.set("tex_coord", inputs.get(VertexInput::TexCoord)?)?;

            Ok(())
        },
        // Fragment shader
        |inputs, parameters, outputs| {
            // Get the tex_coord input
            let tex_coord = inputs.get("tex_coord")?;

            // Get the texture_color texture view
            let color_view = parameters.get::<TextureView>("color_texture");

            // Output color = input color * texture color
            let input_color = inputs.get("color")?;
            let texture_color = color_view.sample(tex_coord, 0.0);
            let output_color = input_color.mul(texture_color);
            outputs.set_fragment_color(output_color);

            Ok(())
        },
    )?;

    // Create the mesh
    graphics_cache.create_mesh(
        Some("mesh"),
        &vertex_layout,
        vec![
            Rectangle::default()
                .with_center(vector!(0.1, 0.2, 0.0))
                .with_rotation_z(0.2)
                .with_size(vector!(1.1, 0.8))
                .with_color(WHITE.lerp(&RED, 0.5)),
            Rectangle::default()
                .with_center(vector!(-0.5, -0.2, 0.0))
                .with_rotation_z(0.5)
                .with_size(vector!(0.9, 1.3))
                .with_color(WHITE.lerp(&GREEN, 0.5)),
            Rectangle::default()
                .with_center(vector!(0.3, -0.4, 0.0))
                .with_rotation_z(0.8)
                .with_size(vector!(1.2, 0.7))
                .with_color(WHITE.lerp(&BLUE, 0.5)),
        ],
    );

    let mut regions = HashMap::new();
    regions.insert(
        "Test".to_string(),
        TextureRegion(vector!(0, 0, 0), vector!(64, 64, 0)),
    );

    // Create the texture.
    graphics_cache.create_texture_from_file(
        Some("texture"),
        TextureType::Color,
        "assets/texture.png",
        Some(regions),
    )?;

    Ok(())
}

// Called when the engine renders a frame
pub fn render(
    _engine: &mut Engine,
    _universe: &mut Universe,
    _async_data: AppData<AsyncData>,
    graphics_cache: &mut GfxCache,
    framebuffer: TargetBuffer,
) -> AppEventResult<()> {
    // Clear color is a mix of blue, gray, and white
    let clear_color = BLUE // Start with blue
        .lerp(&GRAY, 0.5) // Mix 50% with gray
        .lerp(&WHITE, 0.25); // Mix 25% with white

    // Clear the framebuffer with the clear color.
    framebuffer.clear_with_color(clear_color);

    // Retrieve the program and input layout
    let program = graphics_cache.get("program").unwrap();
    let input_layout = graphics_cache.get("input layout").unwrap();

    // Retrieve the mesh
    let mesh = graphics_cache.get("mesh").unwrap();

    // Retrieve a view of the region "test" from the texture
    let texture_view = graphics_cache
        .get_texture("texture")
        .unwrap()
        .region_view("Test")
        .unwrap();

    // Set the parameters for rendering
    let mut parameters = RenderParameters::new();
    parameters.set("color_texture", texture_view);

    // Draw the triangle
    framebuffer.render_mesh(program, input_layout, &parameters, &mesh)?;

    Ok(())
}
