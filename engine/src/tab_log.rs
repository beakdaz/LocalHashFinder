//! Per-tab action journal.

use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_LINES: usize = 400;

pub struct TabLog {
    lines: Mutex<Vec<String>>,
}

impl TabLog {
    pub fn shared() -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            lines: Mutex::new(Vec::new()),
        })
    }

    fn stamp() -> String {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        format!(
            "{:02}:{:02}:{:02}",
            (secs % 86_400) / 3600,
            (secs % 3600) / 60,
            secs % 60
        )
    }

    pub fn push(&self, msg: impl AsRef<str>) {
        let line = format!("[{}] {}", Self::stamp(), msg.as_ref());
        let mut g = self.lines.lock().unwrap();
        g.push(line);
        if g.len() > MAX_LINES {
            let excess = g.len() - MAX_LINES;
            g.drain(0..excess);
        }
    }

    pub fn clear(&self) {
        self.lines.lock().unwrap().clear();
    }

    pub fn text(&self) -> String {
        self.lines.lock().unwrap().join("\n")
    }
}
