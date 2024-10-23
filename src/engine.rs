use std::time::Instant;

/// Manages the engine's state and provides methods for controlling the engine.
/// Meant to be passed to the app.
pub struct Engine {
    /// Whether the engine is stopping.
    stopping: bool,
    /// The time the engine started.
    start_time: Option<Instant>,
    /// The time of the previous iteration.
    last_iteration_time: Option<Instant>,
    /// The time between the previous iteration and the current iteration.
    /// Measured in seconds.
    delta_time: Option<f32>,
}

impl Engine {
    /// Create a new `Engine`.
    pub(crate) fn new() -> Self {
        Self {
            stopping: false,
            start_time: None,
            last_iteration_time: None,
            delta_time: None,
        }
    }

    /// Stop the engine.
    pub fn stop(&mut self) {
        self.stopping = true;
    }

    /// Check if the engine is stopping.
    pub fn is_stopping(&self) -> bool {
        self.stopping
    }

    /// Start a new iteration.
    pub(crate) fn start_iteration(&mut self) {
        // Get the current time.
        let now = Instant::now();

        // Set the start time if it hasn't been set yet.
        if self.start_time.is_none() {
            self.start_time = Some(now);
        }

        // Update delta time and last iteration time.
        self.delta_time = self
            .last_iteration_time
            .map(|last| (now - last).as_secs_f32());
        self.last_iteration_time = Some(now);
    }

    /// Get the time the engine started.
    pub fn start_time(&self) -> Instant {
        self.start_time
            .expect("Method `start_iteration` must be called before `start_time`")
    }

    /// Get the delta time (time between the previous iteration and the current iteration).
    /// Measured in seconds. This function may return 0.0 in some cases.
    pub fn delta_time(&self) -> f32 {
        self.delta_time.unwrap_or(0.0)
    }
}
