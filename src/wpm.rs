use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct WpmCalculator {
    timestamps: VecDeque<Instant>,
    window_duration: Duration,
    last_keypress: Option<Instant>,
    idle_timeout: Duration,
}

impl WpmCalculator {
    pub fn new() -> Self {
        Self {
            timestamps: VecDeque::new(),
            window_duration: Duration::from_secs(3),
            last_keypress: None,
            idle_timeout: Duration::from_secs(1),
        }
    }

    /// Adds a key press timestamp.
    pub fn add_keypress(&mut self) {
        let now = Instant::now();
        self.last_keypress = Some(now);
        self.timestamps.push_back(now);
        self.cleanup_old_timestamps(now);
    }

    /// Removes timestamps older than the window duration.
    fn cleanup_old_timestamps(&mut self, now: Instant) {
        while let Some(&timestamp) = self.timestamps.front() {
            if now.duration_since(timestamp) > self.window_duration {
                self.timestamps.pop_front();
            } else {
                break;
            }
        }
    }

    /// Calculates current WPM based on the number of key presses in the window.
    pub fn calculate_wpm(&mut self) -> f64 {
        let now = Instant::now();

        // If idle for more than the timeout, reset to 0
        if let Some(last) = self.last_keypress {
            if now.duration_since(last) > self.idle_timeout {
                self.timestamps.clear();
                return 0.0;
            }
        } else {
            return 0.0;
        }

        self.cleanup_old_timestamps(now);
        
        let count = self.timestamps.len();
        if count == 0 {
            0.0
        } else {
            let words = count as f64 / 5.0; // 5 chars per word
            let minutes = self.window_duration.as_secs_f64() / 60.0; // Duration in minutes
            words / minutes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_wpm_calculation() {
        let mut calc = WpmCalculator::new();
        
        // Add 5 keystrokes (1 word)
        for _ in 0..5 {
            calc.add_keypress();
        }

        // Immediate check: 
        // 5 chars = 1 word
        // Window = 3s = 0.05 min
        // WPM = 1 / 0.05 = 20
        let wpm = calc.calculate_wpm();
        assert!((wpm - 20.0).abs() < 0.001, "Expected 20 WPM, got {}", wpm);

        // Wait for window to expire + buffer
        thread::sleep(Duration::from_secs(4));
        
        let wpm_after = calc.calculate_wpm();
        assert_eq!(wpm_after, 0.0, "Expected 0 WPM after decay");
    }
}


