pub mod gfx_cache;
pub mod program;
pub mod shader;
pub mod target_buffer;
pub mod vertex_layout;
pub mod vertex_list;
pub mod buffer;

use std::cell::{Cell, RefCell};

use anyhow::Result;
use gfx_cache::GfxCache;
use glfw::Window;
use target_buffer::TargetBuffer;

thread_local! {
    /// The graphics controller. Should only be used in the main thread.
    pub static GFX: Cell<Option<Gfx>> = None.into();
    /// The graphics cache. Should only be used in the main thread.
    pub static CACHE: RefCell<Option<GfxCache>> = None.into();
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
    pub fn use_cache_mut<R>(callback: impl FnOnce(&mut GfxCache) -> R) -> R {
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
