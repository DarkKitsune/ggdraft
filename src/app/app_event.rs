use crate::{
    color,
    geometry::text::{Text, TextAlignment},
    node_class::{MeshRenderer, Viewport},
    node_component::render_component::RenderComponent,
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
    universe: &mut Universe,
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
        Vector::zero(),
        vector!(20, 20),
        vector!(1, 1),
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
        vector!(40.0, 40.0),
        TextAlignment::CENTER,
        font_texture,
        color::WHITE,
        "Hello, world!",
    );

    // Create a mesh from the text object
    graphics_cache.create_mesh(Some("mesh0"), &vertex_layout, &text);

    // Create a viewport node with a default camera
    let viewport_node = universe.create_node(None, Viewport::new_default());

    // Add a mesh renderer node to the viewport node
    universe.create_node(
        Some(&viewport_node),
        MeshRenderer::new(
            Orientation::new_orthographic(Vector::zero(), 0.0),
            graphics_cache.handle("mesh0"),
            graphics_cache.handle("input layout"),
            graphics_cache.handle("program"),
            RenderParameters::new()
            // Pass the font texture to the parameters
                .with(
                    "font_texture",
                    graphics_cache.get_texture("font_texture")
                        .unwrap()
                        .full_view()
                ),
        ),
    );

    println!("Render initialized.");

    Ok(())
}

// Called when the engine renders a frame
pub fn render(
    _engine: &mut Engine,
    universe: &mut Universe,
    _async_data: AppData<AsyncData>,
    graphics_cache: &mut GfxCache,
    framebuffer: TargetBuffer,
    framebuffer_size: Vector2<u32>,
) -> AppEventResult<()> {
    // Set the viewport to cover the entire framebuffer
    unsafe {
        framebuffer.__set_viewport(vector!(0.5, 0.5), vector!(1.0, 1.0), framebuffer_size);
    }

    // Clear the framebuffer with a color
    framebuffer.clear_with_color(
        BLUE // Start with blue
            .lerp(&GRAY, 0.5) // Mix 50% with gray
            .lerp(&WHITE, 0.25), // Mix 25% with white
    );

    // Clear the framebuffer depth
    framebuffer.clear_depth();

    
    // Find all viewport nodes in the universe
    let viewport_nodes = universe.nodes().with_class::<Viewport>();

    // For each viewport, render its contents with the appropriate viewport settings
    // and camera
    for node in viewport_nodes {
        // Get the viewport information
        let viewport = node.class_as::<Viewport>().unwrap();
        let camera = viewport.camera();

        // Set the viewport in the state
        unsafe {
            framebuffer.__set_viewport(viewport.center(), viewport.size(), framebuffer_size);
        }

        // Find all children of the viewport node that have a RenderComponent
        let children = universe
            .nodes_with_handles(node.children())
            .flatten()
            .with_component::<RenderComponent>();

        // For each child node, render it
        for (child, render_component) in children {
            // Render the child node using its render component
            render_component.render(
                &child,
                &framebuffer,
                framebuffer_size,
                camera, true,
                graphics_cache,
                Some(universe)
            );
        }
    }
    Ok(())
}
