use std::rc::Rc;

use crate::gfx::{
    texture::TextureView,
    vertex_layout::{VertexInput, VertexLayout},
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
            .with_input(VertexInput::Color)
            .with_input(VertexInput::TexCoord),
    );

    // Create a mesh from a rectangle shape
    graphics_cache.create_mesh(
        "mesh",
        vertex_layout.clone(),
        &vec![
            Rectangle::default()
                .with_center(vector!(0.1, 0.2, 0.0))
                .with_rotation_z(0.2)
                .with_size(vector!(1.1, 0.8))
                .with_color(WHITE.lerp(&RED, 0.5))
                .with_tex_coords(vector!(0.0, 0.0), vector!(1.0, 1.0)),
            Rectangle::default()
                .with_center(vector!(-0.5, -0.2, 0.0))
                .with_rotation_z(0.5)
                .with_size(vector!(0.9, 1.3))
                .with_color(WHITE.lerp(&GREEN, 0.5))
                .with_tex_coords(vector!(0.0, 0.0), vector!(0.5, 1.0)),
            Rectangle::default()
                .with_center(vector!(0.3, -0.4, 0.0))
                .with_rotation_z(0.8)
                .with_size(vector!(1.2, 0.7))
                .with_color(WHITE.lerp(&BLUE, 0.5))
                .with_tex_coords(vector!(0.5, 0.0), vector!(1.0, 1.0)),
        ],
    );

    // Create a texture from a file
    graphics_cache.create_texture_from_file("texture", TextureUnit::Color, "assets/texture.png")?;

    // Create a vertex array from the vertex layout
    graphics_cache.create_input_layout_from_vertex_layout("input layout", vertex_layout);

    // Create shader program
    graphics_cache.create_program_vertex_fragment(
        // Name of the program
        "program",
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

            // Get the texture_color texture
            let color_texture = parameters.get::<TextureView>("color_texture");

            // Output color = input color * texture color
            let input_color = inputs.get("color")?;
            let texture_color = color_texture.sample(tex_coord, 0);
            let output_color = input_color.mul(texture_color);
            outputs.set_fragment_color(output_color);

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

    // Retrieve the program, mesh, and input layout
    let program = graphics_cache.get("program").unwrap();
    let mesh = graphics_cache.get("mesh").unwrap();
    let input_layout = graphics_cache.get("input layout").unwrap();

    // Retrieve a view of the texture
    let texture_view = graphics_cache.get_texture_view("texture").unwrap();

    // Set the parameters for rendering
    let mut parameters = InputParameters::new();
    parameters.set("color_texture", texture_view);

    // Draw the triangle
    framebuffer.render_mesh(program, mesh, input_layout, &parameters)?;

    Ok(())
}
