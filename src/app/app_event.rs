use crate::{
    color,
    geometry::{orientation::Orientation, text::{Text, TextAlignment}}, gfx::render_camera::RenderCamera,
};

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
    let vertex_layout =
        graphics_cache.create_vertex_layout(Some("vertex layout"), Text::build_vertex_layout);

    // Create an input layout from the vertex layout
    graphics_cache.create_input_layout_from_vertex_layout(Some("input layout"), &vertex_layout);

    // Create shader program
    graphics_cache.create_program_vertex_fragment(
        Some("program"),
        "input layout",
        Text::vertex_shader,
        Text::fragment_shader,
    )?;

    // Create a glyph map
    let glyphs = Text::build_glyph_map(
        // TODO: Expand this list if decide to keep this code
        [
            " !\"#$%&'()*+,-.",
            "/0123456789:;<=",
            ">?@ABCDEFGHIJKL",
            "MNOPQRSTUVWXYZ[",
            "\\]^_`abcdefghij",
            "klmnopqrstuvwxy",
            "z{|}~",
        ],
        vector!(0, 0),
        vector!(20, 20),
        vector!(2, 2),
    );

    // Load a font texture
    let font_texture = graphics_cache.create_texture_from_file(
        Some("font_texture"),
        TextureType::Color,
        "assets/ascii.png",
        None,
        Some(glyphs),
    )?;

    // Create a text object
    let text = Text::new(
        Vector::zero(),
        Quaternion::identity(),
        vector!(40.0; 2),
        TextAlignment::CENTER,
        font_texture,
        color::WHITE,
        "Hello, world!",
    );

    // Create a mesh from the text object
    graphics_cache.create_mesh(Some("mesh0"), &vertex_layout, &text);

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

    // Begin the parameters for rendering
    let mut parameters = RenderParameters::new();

    // Set the model matrix
    parameters.set_model_matrix(Matrix::new_translation(&vector!(128.0, 128.0, 0.0)));
    
    // Set the camera
    parameters.set_camera(
        framebuffer_size.convert_to().unwrap(),
        &RenderCamera::orthographic(
            Orientation::new_orthographic(
                Vector::zero(),
                0.0
            ),
            -1.0,
            1.0
        ),
    );

    // Set the font texture
    parameters.set(
        "font_texture",
        graphics_cache
            .get_texture("font_texture")
            .unwrap()
            .full_view(),
    );

    // Draw the triangle
    framebuffer.render_mesh(program, input_layout, &parameters, &mesh0)?;

    Ok(())
}
