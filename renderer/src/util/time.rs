use std::time::{Duration, Instant};

pub struct FrameTimer {
    last: Instant,
    frame_count: u64,
    accum: Duration,
    min_frame: Duration,
    max_frame: Duration,
}

impl FrameTimer {
    pub fn new() -> Self {
        Self {
            last: Instant::now(),
            frame_count: 0,
            accum: Duration::ZERO,
            min_frame: Duration::MAX,
            max_frame: Duration::ZERO,
        }
    }

    pub fn tick(&mut self) -> Duration {
        let now = Instant::now();
        let delta = now.duration_since(self.last);
        self.last = now;
        self.accum += delta;
        self.frame_count += 1;
        if delta < self.min_frame {
            self.min_frame = delta;
        }
        if delta > self.max_frame {
            self.max_frame = delta;
        }
        delta
    }

    pub fn average_fps(&self) -> f32 {
        if self.accum.is_zero() || self.frame_count == 0 {
            return 0.0;
        }
        let secs = self.accum.as_secs_f32();
        self.frame_count as f32 / secs
    }

    pub fn min_frame_ms(&self) -> f32 {
        if self.min_frame == Duration::MAX {
            0.0
        } else {
            self.min_frame.as_secs_f32() * 1000.0
        }
    }

    pub fn max_frame_ms(&self) -> f32 {
        self.max_frame.as_secs_f32() * 1000.0
    }

    pub fn average_frame_ms(&self) -> f32 {
        if self.frame_count == 0 {
            return 0.0;
        }
        (self.accum.as_secs_f32() * 1000.0) / self.frame_count as f32
    }

    pub fn reset_stats(&mut self) {
        self.frame_count = 0;
        self.accum = Duration::ZERO;
        self.min_frame = Duration::MAX;
        self.max_frame = Duration::ZERO;
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }
}

impl Default for FrameTimer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_timer_initial_state() {
        let t = FrameTimer::new();
        assert_eq!(t.frame_count(), 0);
        assert_eq!(t.average_fps(), 0.0);
        assert_eq!(t.average_frame_ms(), 0.0);
        assert_eq!(t.min_frame_ms(), 0.0);
    }

    #[test]
    fn test_timer_ticks() {
        let mut t = FrameTimer::new();
        let d1 = t.tick();
        assert!(d1.as_nanos() > 0);
        sleep(Duration::from_millis(5));
        let d2 = t.tick();
        assert!(d2.as_millis() >= 4);
        assert_eq!(t.frame_count(), 2);
    }

    #[test]
    fn test_reset_stats() {
        let mut t = FrameTimer::new();
        t.tick();
        t.tick();
        assert_eq!(t.frame_count(), 2);
        t.reset_stats();
        assert_eq!(t.frame_count(), 0);
        assert_eq!(t.average_frame_ms(), 0.0);
    }

    #[test]
    fn test_min_max_tracked() {
        let mut t = FrameTimer::new();
        sleep(Duration::from_millis(2));
        t.tick();
        sleep(Duration::from_millis(10));
        t.tick();
        assert!(t.max_frame_ms() >= 9.0);
        assert!(t.min_frame_ms() >= 1.0);
        assert!(t.max_frame_ms() > t.min_frame_ms());
    }
}
