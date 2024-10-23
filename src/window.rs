use ggmath::prelude::*;
use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};

use crate::gfx::Gfx;

/// Create the window.
pub(crate) fn create_window() -> (Glfw, PWindow, GlfwReceiver<(f64, WindowEvent)>) {
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
    window.set_close_polling(true);
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
    glfw: &mut Glfw,
    events: &GlfwReceiver<(f64, WindowEvent)>,
) -> WindowEvents {
    // Poll for events in the window.
    glfw.poll_events();

    // Handle events in the window.
    let mut window_events = WindowEvents::new();
    for (_, event) in glfw::flush_messages(events) {
        match event {
            // Window close event or Escape key press event.
            WindowEvent::Close => {
                window_events.push_close();
            },

            // Window resize event.
            WindowEvent::FramebufferSize(width, height) => {
                window_events.push_resize(vector!(width as u32, height as u32));
            },

            // Key press event.
            WindowEvent::Key(key, _, Action::Press, _) => {
                window_events.push_key_press(key);
            },

            // Key release event.
            WindowEvent::Key(key, _, Action::Release, _) => {
                window_events.push_key_release(key);
            },

            // Ignore other events.
            _ => {}
        }
    }

    window_events
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

/// Window events that were caught by handle_window_events.
/// This is meant to be passed to the app.
#[derive(Debug)]
pub struct WindowEvents {
    closed: bool,
    resized: Option<Vector2<u32>>,
    key_presses: Vec<Key>,
    key_releases: Vec<Key>,
}

impl WindowEvents {
    /// Create a new `WindowEvents`.
    pub(crate) fn new() -> Self {
        Self { closed: false, resized: None, key_presses: Vec::new(), key_releases: Vec::new() }
    }

    /// Push a window close event.
    pub(crate) fn push_close(&mut self) {
        self.closed = true;
    }

    /// Push a window resize event.
    pub(crate) fn push_resize(&mut self, size: Vector2<u32>) {
        self.resized = Some(size);
    }

    /// Push a key press event.
    pub(crate) fn push_key_press(&mut self, key: Key) {
        self.key_presses.push(key);
    }

    /// Push a key release event.
    pub(crate) fn push_key_release(&mut self, key: Key) {
        self.key_releases.push(key);
    }

    /// Check if the window was closed.
    pub fn closed(&self) -> bool {
        self.closed
    }

    /// Get the window resize event if one occurred.
    pub fn resized(&self) -> Option<Vector2<u32>> {
        self.resized
    }

    /// Check if a given key was pressed.
    pub fn key_pressed(&self, key: Key) -> bool {
        self.key_presses.contains(&key)
    }

    /// Check if a given key was released.
    pub fn key_released(&self, key: Key) -> bool {
        self.key_releases.contains(&key)
    }
    
    /// Get the key presses that occurred.
    pub fn key_presses(&self) -> &[Key] {
        &self.key_presses
    }

    /// Get the key releases that occurred.
    pub fn key_releases(&self) -> &[Key] {
        &self.key_releases
    }
}