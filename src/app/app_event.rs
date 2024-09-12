use super::app_prelude::*;

// Initialization logic.
pub fn init(_app_data: AppData<Data>) -> AppEventResult<()> {
    println!("App has been initialized.");
    Ok(())
}

// Logic to run before the engine thinks.
pub fn pre_think(_app_data: AppData<Data>) -> AppEventResult<()> {
    Ok(())
}

// Logic to run after the engine thinks.
pub fn post_think(_app_data: AppData<Data>) -> AppEventResult<()> {
    Ok(())
}

// Render
pub fn render(_app_data: AppData<Data>, framebuffer: TargetBuffer) -> AppEventResult<()> {
    let clear_color = color::BLUE // Start with blue
        .lerp(&color::GRAY, 0.5) // Mix 50% with gray
        .lerp(&color::WHITE, 0.25); // Mix 25% with white

    // Clear the framebuffer with the clear color.
    framebuffer.clear_with_color(clear_color);

    Ok(())
}
