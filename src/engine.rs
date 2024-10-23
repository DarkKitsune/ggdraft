/// Manages the engine's state and provides methods for controlling the engine.
/// Meant to be passed to the app.
pub struct Engine {
    /// Whether the engine is stopping.
    stopping: bool,
}

impl Engine {
    /// Create a new `Engine`.
    pub(crate) fn new() -> Self {
        Self { stopping: false }
    }

    /// Stop the engine.
    pub fn stop(&mut self) {
        self.stopping = true;
    }

    /// Check if the engine is stopping.
    pub fn is_stopping(&self) -> bool {
        self.stopping
    }
}
