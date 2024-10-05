use super::app_prelude::*;

// Called when the app is initialized
pub fn init(_app_data: AppData<Data>) -> AppEventResult<()> {
    println!("App has been initialized.");
    Ok(())
}

// Called when initializing the rendering engine
pub fn graphics_init(_app_data: AppData<Data>, cache: &mut GfxCache) -> AppEventResult<()> {
    cache.insert("A", 1);
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
pub fn render(_app_data: AppData<Data>, cache: &mut GfxCache, framebuffer: TargetBuffer) -> AppEventResult<()> {
    let clear_color = color::BLUE // Start with blue
        .lerp(&color::GRAY, 0.5) // Mix 50% with gray
        .lerp(&color::WHITE, 0.25); // Mix 25% with white

    // Clear the framebuffer with the clear color.
    framebuffer.clear_with_color(clear_color);

    // Print the cached value of "A".
    println!("Cached value of 'A': {}", cache.get::<i32>("A").unwrap());

    Ok(())
}
