use app_weaver::prelude::*;
use glfw::{Context, Glfw, GlfwReceiver, PWindow};

use crate::{app::WindowMessage, gfx::Gfx};

/// Create the window.
pub(crate) fn create_window() -> (Glfw, PWindow, GlfwReceiver<(f64, glfw::WindowEvent)>) {
    // Initialize GLFW.
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    // Use an OpenGL 4.5 core profile.
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // Create the window.
    let (mut window, events) = glfw
        .create_window(800, 600, "Hello, World!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Poll for events in the window.
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Make the window the current OpenGL context.
    window.make_current();

    // Init graphics controller.
    Gfx::init(&mut window);

    // Return the GLFW context, window, and event receiver.
    (glfw, window, events)
}

/// Handle window events.
pub(crate) fn handle_window_events(
    window_channel: &Channel,
    glfw: &mut Glfw,
    events: &GlfwReceiver<(f64, glfw::WindowEvent)>,
) {
    // Poll for events in the window.
    glfw.poll_events();

    // Handle events in the window.
    for (_, event) in glfw::flush_messages(events) {
        match event {
            // Resize the viewport when the window is resized.
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height);
            },

            // Close the window when the escape key is pressed.
            glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                window_channel.send(WindowMessage::Close);
            }

            // Ignore other events.
            _ => {}
        }
    }
}

/// Swap the window frame buffers.
pub(crate) fn swap_window_buffers(window: &mut PWindow) {
    // Swap the window frame buffers.
    window.swap_buffers();

    // Clear the window frame buffer.
    unsafe {
        // Bind the default framebuffer.
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        // Set the clear color to cornflower blue.
        gl::ClearColor(0.392, 0.584, 0.929, 1.0);
        // Clear the color buffer.
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}
