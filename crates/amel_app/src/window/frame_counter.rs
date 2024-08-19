pub struct FrameCounter {
    // Instant of the last time we printed the frame time.
    last_update_instant: web_time::Instant,
    // Number of frames since the last time we printed the frame time.
    frame_count: u32,
}

impl FrameCounter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update(&mut self) {
        self.frame_count += 1;
    }

    pub fn elapsed_secs(&self) -> f32 {
        let now = web_time::Instant::now();
        (now - self.last_update_instant).as_secs_f32()
    }

    pub fn frame_count(&self) -> u32 {
        self.frame_count
    }

    pub fn duration(&self) -> web_time::Duration {
        web_time::Instant::now().duration_since(self.last_update_instant)
    }

    pub fn fps(&self) -> f32 {
        let elapsed_secs = self.elapsed_secs();
        if elapsed_secs > 0.0 {
            self.frame_count as f32 / elapsed_secs
        } else {
            0.0
        }
    }
}

impl Default for FrameCounter {
    fn default() -> Self {
        Self {
            last_update_instant: web_time::Instant::now(),
            frame_count: 0,
        }
    }
}
