pub mod buffer;
pub mod gfx_cache;
pub mod input_layout;
pub mod program;
pub mod shader;
pub mod target_buffer;
pub mod vertex_layout;
pub mod vertex_list;

use std::{
    cell::{Cell, RefCell},
    os::raw::c_void,
};

use buffer::{IndexBuffer, VertexBuffer};
use gfx_cache::GfxCache;
use gl::types::{GLchar, GLenum, GLsizei, GLuint};
use glfw::Window;
use input_layout::{InputLayout, VERTEX_BUFFER_LOCATION};
use target_buffer::TargetBuffer;

thread_local! {
    /// The graphics controller. Should only be used in the main thread.
    pub static GFX: Cell<Option<Gfx>> = None.into();
    /// The graphics cache. Should only be used in the main thread.
    pub static CACHE: RefCell<Option<GfxCache>> = None.into();
}

/// The debug message callback for OpenGL.
extern "system" fn debug_message_callback(
    source: GLenum,
    error_type: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut c_void,
) {
    unsafe {
        // Convert source to a string.
        let source = match source {
            gl::DEBUG_SOURCE_API => "API",
            gl::DEBUG_SOURCE_WINDOW_SYSTEM => "WINDOW_SYSTEM",
            gl::DEBUG_SOURCE_SHADER_COMPILER => "SHADER_COMPILER",
            gl::DEBUG_SOURCE_THIRD_PARTY => "THIRD_PARTY",
            gl::DEBUG_SOURCE_APPLICATION => "APPLICATION",
            gl::DEBUG_SOURCE_OTHER => "OTHER",
            _ => "UNKNOWN",
        };

        // Convert error_type to a string.
        let (error_type, should_panic) = match error_type {
            gl::DEBUG_TYPE_ERROR => ("ERROR", true),
            gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => ("DEPRECATED_BEHAVIOR", false),
            gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => ("UNDEFINED_BEHAVIOR", true),
            gl::DEBUG_TYPE_PORTABILITY => ("PORTABILITY", false),
            gl::DEBUG_TYPE_PERFORMANCE => ("PERFORMANCE", false),
            gl::DEBUG_TYPE_MARKER => ("MARKER", false),
            gl::DEBUG_TYPE_PUSH_GROUP => ("PUSH_GROUP", false),
            gl::DEBUG_TYPE_POP_GROUP => ("POP_GROUP", false),
            gl::DEBUG_TYPE_OTHER => ("OTHER", false),
            _ => ("UNKNOWN", false),
        };

        // Convert severity to a string.
        let severity = match severity {
            gl::DEBUG_SEVERITY_HIGH => "HIGH",
            gl::DEBUG_SEVERITY_MEDIUM => "MEDIUM",
            gl::DEBUG_SEVERITY_LOW => "LOW",
            gl::DEBUG_SEVERITY_NOTIFICATION => "NOTIFICATION",
            _ => "UNKNOWN",
        };

        // Get the message as a string.
        let message = std::slice::from_raw_parts(message as *const u8, length as usize);
        let message = std::str::from_utf8_unchecked(message);

        // Decorate the message with the source, error type, id, and severity.
        let message = format!(
            "{}: {}: {}: {}: {}",
            source, error_type, id, severity, message
        );

        // Print the message.
        if should_panic {
            panic!("{}", message);
        } else {
            eprintln!("{}", message);
        }
    }
}

/// The graphics controller, an entry point for all graphics operations.
#[derive(Debug, Clone, Copy)]
pub struct Gfx;

impl !Send for Gfx {}
impl !Sync for Gfx {}

impl Gfx {
    /// Initialize the graphics controller.
    /// This should only be called once in the main thread.
    /// This function will panic if called more than once.
    pub fn init(window: &mut Window) {
        // Get a thread-local reference to the graphics controller.
        GFX.with(|gfx| {
            // Panic if the graphics controller has already been initialized.
            if gfx.get().is_some() {
                panic!("Graphics controller has already been initialized.");
            }

            // Load the OpenGL function pointers.
            gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

            // Enable debug output.
            unsafe {
                gl::Enable(gl::DEBUG_OUTPUT);
                gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
                gl::DebugMessageCallback(Some(debug_message_callback), std::ptr::null());
                gl::DebugMessageControl(
                    gl::DONT_CARE,
                    gl::DONT_CARE,
                    gl::DONT_CARE,
                    0,
                    std::ptr::null(),
                    gl::TRUE,
                );
            }

            // Initialize the graphics controller.
            gfx.set(Some(Gfx));
        });

        // Get a thread-local reference to the graphics cache.
        CACHE.with(|cache| {
            // Initialize the graphics cache.
            cache.replace(Some(GfxCache::new()));
        });
    }

    /// Try to get the graphics controller.
    /// This function will return `None` if the graphics controller has not been initialized,
    /// or if it is called from a thread other than the main thread.
    pub fn try_get() -> Option<Gfx> {
        let mut result = None;
        // Get a thread-local reference to the graphics controller.
        GFX.with(|gfx| {
            // Get the graphics controller.
            result = gfx.get();
        });
        result
    }

    /// Get a reference to the graphics controller.
    /// This function will panic if the graphics controller is not available.
    pub fn get() -> Gfx {
        Gfx::try_get().expect("Graphics controller not available.")
    }

    /// Executes the callback with a mutable reference to the graphics cache.
    pub fn use_cache_mut<R>(&self, callback: impl FnOnce(&mut GfxCache) -> R) -> R {
        // Get a thread-local reference to the graphics cache.
        CACHE.with(|cache| {
            // Get a mutable reference to the thread-local Option<GfxCache>.
            let mut cache = cache.borrow_mut();
            // Get a mutable reference to the GfxCache.
            let cache = cache.as_mut().expect("Graphics cache not available.");
            // Execute the callback with the GfxCache.
            callback(cache)
        })
    }

    /// Get the default framebuffer as a `TargetBuffer`.
    pub fn default_framebuffer(&self) -> TargetBuffer {
        TargetBuffer::DEFAULT
    }
}
