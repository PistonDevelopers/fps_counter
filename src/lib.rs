//! A Frames Per Second counter.

extern crate time;

use std::collections::VecDeque;

/// Measures Frames Per Second (FPS).
pub struct FPSCounter {
    /// The last registered frames.
    last_second_frames: VecDeque<time::Timespec>
}

impl FPSCounter {
    /// Creates a new FPSCounter.
    pub fn new() -> FPSCounter {
        FPSCounter {
            last_second_frames: VecDeque::with_capacity(128)
        }
    }

    /// Updates the FPSCounter and returns number of frames.
    pub fn tick(&mut self) -> usize {
        let now = time::now().to_timespec();
        let a_second_ago = time::Timespec::new(now.sec - 1, now.nsec);

        while self.last_second_frames.front().map_or(false, |t| *t < a_second_ago) {
            self.last_second_frames.pop_front();
        }

        self.last_second_frames.push_back(now);
        self.last_second_frames.len()
    }
}
