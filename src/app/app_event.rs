use std::rc::Rc;

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
    // Create vertex layout describing the vertices going into the shader
    let vertex_layout = Rc::new(
        VertexLayout::new()
            .with_input(VertexInput::Position)
            .with_input(VertexInput::Color),
    );

    // Create a mesh from a rectangle shape
    graphics_cache.create_mesh(
        "mesh",
        vertex_layout.clone(),
        &vec![
            Rectangle::default()
                .with_center(vector!(0.1, 0.2, 0.0))
                .with_rotation_z(0.2)
                .with_size(vector!(0.4, 0.7))
                .with_color(BLUE),
            Rectangle::default()
                .with_center(vector!(0.6, 0.7, 0.0))
                .with_rotation_z(0.5)
                .with_size(vector!(0.3, 0.3))
                .with_color(RED),
            Rectangle::default()
                .with_center(vector!(-0.8, 0.3, 0.0))
                .with_rotation_z(0.7)
                .with_size(vector!(1.5, 0.5))
                .with_color(GREEN),
        ],
    );

    // Create a vertex array from the vertex layout
    graphics_cache.create_input_layout_from_vertex_layout("input layout", vertex_layout);

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
    let input_layout = graphics_cache.get("input layout").unwrap();
    let mesh = graphics_cache.get("mesh").unwrap();
    framebuffer.render_mesh(program, mesh, input_layout)?;

    Ok(())
}
