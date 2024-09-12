pub struct Data {
    closed: bool,
}

impl Data {
    pub fn new() -> Self {
        Self { closed: false }
    }

    pub fn close(&mut self) {
        self.closed = true;
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }
}
