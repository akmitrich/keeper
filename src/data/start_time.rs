#[derive(Debug)]
pub struct StartTime {
    inner: std::time::SystemTime,
}

impl StartTime {
    pub fn new() -> Self {
        Self {
            inner: std::time::SystemTime::now(),
        }
    }

    pub fn alive(&self) -> std::time::Duration {
        std::time::SystemTime::now()
            .duration_since(self.inner)
            .unwrap_or_default()
    }

    pub fn start_time(&self) -> std::time::SystemTime {
        self.inner
    }
}

impl Default for StartTime {
    fn default() -> Self {
        Self::new()
    }
}
