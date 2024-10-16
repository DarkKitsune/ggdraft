use crate::gfx::{
    vertex_layout::{VertexInput, VertexLayout},
    vertex_list::VertexList,
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
    // Create a rectangle
    let rectangle = Rectangle::new_z(Vector3::zero(), vector!(1.0, 0.5), RED);

    // Create vertex layout describing the vertices going into the shader
    let layout = VertexLayout::new()
        .with_input(VertexInput::Position)
        .with_input(VertexInput::Color);

    // Create vertex list
    let vertex_list = VertexList::from_shape(layout.clone(), &rectangle)?;

    // Create a vertex and index buffer from the vertex list
    graphics_cache.create_vertex_buffer("vertex buffer", &vertex_list);
    graphics_cache.create_buffer_from_slice("index buffer", vertex_list.indices().unwrap());

    // Create a vertex array from the vertex layout
    graphics_cache.create_input_layout_from_vertex_layout("input layout", layout);

    // Create shader program
    graphics_cache.create_program_vertex_fragment(
        // Name of the program
        "program",
        // Name of the input layout to use
        "input layout",
        // Vertex shader
        |input, output| {
            // Set the vertex position to the input position
            output.set_vertex_position(input.get(VertexInput::Position)?.append(1.0));

            // Set the color to the input color, or white if no color is provided
            output.set(
                "color",
                input.get(VertexInput::Color).unwrap_or(WHITE.into()),
            )?;

            Ok(())
        },
        // Fragment shader
        |input, output| {
            // Set the output color to the input color
            output.set_fragment_color(input.get("color")?);

            Ok(())
        },
    )?;

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
    let clear_color = BLUE // Start with blue
        .lerp(&GRAY, 0.5) // Mix 50% with gray
        .lerp(&WHITE, 0.25); // Mix 25% with white

    // Clear the framebuffer with the clear color.
    framebuffer.clear_with_color(clear_color);

    // Draw the triangle
    let program = graphics_cache.get("program").unwrap();
    let vertex_buffer = graphics_cache.get("vertex buffer").unwrap();
    let index_buffer = graphics_cache.get("index buffer").unwrap();
    let input_layout = graphics_cache.get("input layout").unwrap();
    framebuffer.render_triangles(program, vertex_buffer, index_buffer, input_layout)?;

    Ok(())
}
