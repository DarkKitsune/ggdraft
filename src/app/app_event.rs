use crate::gfx::{
    vertex_layout::{VertexInput, VertexLayout},
    vertex_list::{VertexList, VertexListInput},
};

use super::app_prelude::*;

// Called when the app is initialized
pub fn init(_app_data: AppData<Data>) -> AppEventResult<()> {
    println!("App has been initialized.");
    Ok(())
}

// Called when initializing the rendering engine
pub fn graphics_init(
    _app_data: AppData<Data>,
    graphics_cache: &mut GfxCache,
) -> AppEventResult<()> {
    // Create default shader program
    graphics_cache.create_program_default("program");

    // Create triangle vertices
    let positions = vec![
        vector!(0.0, 0.5, 0.0),
        vector!(-0.5, -0.5, 0.0),
        vector!(0.5, -0.5, 0.0),
    ];
    let colors = vec![color::RED, color::GREEN, color::BLUE];
    let indices = vec![0, 1, 2];

    // Create vertex layout describing the vertices going into the shader
    let layout = VertexLayout::new()
        .with_input(VertexInput::Position)
        .with_input(VertexInput::Color);

    // Create vertex list
    let vertex_list = VertexList::new(
        layout.clone(),
        &[
            VertexListInput::Position(&positions),
            VertexListInput::Color(&colors),
        ],
        Some(indices),
    )
    .unwrap();

    // Create a vertex and index buffer from the vertex list
    graphics_cache.create_vertex_buffer("vertex buffer", &vertex_list);
    graphics_cache.create_buffer_from_slice("index buffer", vertex_list.indices().unwrap());

    // Create a vertex array from the vertex layout
    graphics_cache.create_input_layout_from_vertex_layout("input layout", layout);

    Ok(())
}

// Called before the engine thinks
pub fn pre_think(_app_data: AppData<Data>) -> AppEventResult<()> {
    Ok(())
}

// Called after the engine thinks
pub fn post_think(_app_data: AppData<Data>) -> AppEventResult<()> {
    Ok(())
}

// Called when the engine renders a frame
pub fn render(
    _app_data: AppData<Data>,
    graphics_cache: &mut GfxCache,
    framebuffer: TargetBuffer,
) -> AppEventResult<()> {
    let clear_color = color::BLUE // Start with blue
        .lerp(&color::GRAY, 0.5) // Mix 50% with gray
        .lerp(&color::WHITE, 0.25); // Mix 25% with white

    // Clear the framebuffer with the clear color.
    framebuffer.clear_with_color(clear_color);

    // Draw the triangle
    let program = graphics_cache.get("program").unwrap();
    let vertex_buffer = graphics_cache.get("vertex buffer").unwrap();
    let index_buffer = graphics_cache.get("index buffer").unwrap();
    let input_layout = graphics_cache.get("input layout").unwrap();
    framebuffer.render_triangles(program, vertex_buffer, index_buffer, input_layout, 3)?;

    Ok(())
}
