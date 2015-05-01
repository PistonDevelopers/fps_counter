//! A Frames Per Second counter.

extern crate clock_ticks;

use std::collections::VecDeque;

/// Measures Frames Per Second (FPS).
pub struct FPSCounter {
    /// The last registered frames.
    last_second_frames: VecDeque<u64>
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
        let now = clock_ticks::precise_time_ns();
        let a_second_ago = now - 1_000_000_000;

        while self.last_second_frames.front().map_or(false, |t| *t < a_second_ago) {
            self.last_second_frames.pop_front();
        }

        self.last_second_frames.push_back(now);
        self.last_second_frames.len()
    }
}
