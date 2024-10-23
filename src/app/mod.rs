pub mod app_event;
pub mod app_prelude;
pub mod async_data;

use crate::{engine::Engine, window};
use anyhow::Result;
use app_prelude::*;
// use app_weaver::prelude::*;
use async_data::AsyncData;

/*
#[derive(Clone)]
pub enum WindowMessage {
    Close,
}
impl_message!(WindowMessage);


modules! {
    Main(Data) {
        channels {
            window_channel,
        }

        main(data: AppData<Data>, window_messages(window_channel): &[WindowMessage]) {
            // Handle messages for the window.
            for message in window_messages {
                match message {
                    WindowMessage::Close => {
                        println!("Window has been closed.");
                        data.get_mut().close();
                    },
                }
            }

            // If the window is closed, return early.
            if data.get().is_closed() {
                return Ok(());
            }

            Ok(())
        }
    }
}*/

pub async fn run() -> Result<()> {
    // Create the data which will be shared asynchronously between app modules.
    let async_data = AppData::new(AsyncData::new());
    // Create the app.
    // let app = AppBuilder::new(app_data.clone()).with_module(&Main).build();

    // Create the engine controller.
    let mut engine = Engine::new();

    // Create the window.
    let (mut glfw, mut window, events) = window::create_window();

    // Create the main universe.
    let mut universe = Universe::new();

    // Run init event
    app_event::init(&mut engine, &mut universe, async_data.clone())?;

    // Run init render event
    Gfx::get().use_cache_mut(|cache| {
        app_event::init_render(&mut engine, &mut universe, async_data.clone(), cache)
    })?;

    // Run the app on a loop until the app is closed.
    loop {
        // Start an engine iteration.
        engine.start_iteration();

        // Check for window events.
        let events = window::get_window_events(&mut glfw, &events);

        // Pass the window events to the app.
        Gfx::get().use_cache_mut(|cache| {
            app_event::window_events(
                &mut engine,
                &mut universe,
                async_data.clone(),
                cache,
                &events,
            )
        })?;

        // Run app pre-think event.
        app_event::pre_think(&mut engine, &mut universe, async_data.clone())?;

        // Let the engine think by running the app modules once.
        // app.run().await?;

        // End the loop if the window is closed.
        // This is done after thinking so that the app can clean up.
        if engine.is_stopping() {
            break;
        }

        // Run app post-think event.
        app_event::post_think(&mut engine, &mut universe, async_data.clone())?;

        // Run app render event.
        Gfx::get().use_cache_mut(|cache| {
            app_event::render(
                &mut engine,
                &mut universe,
                async_data.clone(),
                cache,
                Gfx::get().default_framebuffer(),
            )
        })?;

        // Swap the window frame buffers.
        window::swap_window_buffers(&mut window);
    }
    Ok(())
}

pub type AppEventError = anyhow::Error;
pub type AppEventResult<T> = Result<T, AppEventError>;
