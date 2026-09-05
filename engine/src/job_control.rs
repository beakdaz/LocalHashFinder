//! Stop / pause for long-running jobs.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct JobControl {
    stop: AtomicBool,
    pause: AtomicBool,
}

impl JobControl {
    pub fn new_shared() -> Arc<Self> {
        Arc::new(Self {
            stop: AtomicBool::new(false),
            pause: AtomicBool::new(false),
        })
    }

    pub fn reset(&self) {
        self.stop.store(false, Ordering::Relaxed);
        self.pause.store(false, Ordering::Relaxed);
    }

    pub fn request_stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
        self.pause.store(false, Ordering::Relaxed);
    }

    pub fn toggle_pause(&self) {
        self.pause.fetch_xor(true, Ordering::Relaxed);
    }

    pub fn is_paused(&self) -> bool {
        self.pause.load(Ordering::Relaxed)
    }

    pub fn is_stopped(&self) -> bool {
        self.stop.load(Ordering::Relaxed)
    }

    /// Returns `true` when the job should abort.
    pub fn checkpoint(&self) -> bool {
        while self.pause.load(Ordering::Relaxed) {
            if self.stop.load(Ordering::Relaxed) {
                return true;
            }
            thread::sleep(Duration::from_millis(100));
        }
        self.stop.load(Ordering::Relaxed)
    }
}

pub fn checkpoint(control: Option<&JobControl>) -> bool {
    control.map(JobControl::checkpoint).unwrap_or(false)
}
