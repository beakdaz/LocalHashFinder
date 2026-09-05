use std::fs::OpenOptions;
use std::io::Write;
use std::panic;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

use tracing_subscriber::EnvFilter;

use crate::config;

static LOG_PATH: OnceLock<PathBuf> = OnceLock::new();

pub fn log_path() -> PathBuf {
    LOG_PATH
        .get()
        .cloned()
        .unwrap_or_else(|| config::exe_dir().join("LocalHashFinder.log"))
}

struct FileWriter(Arc<Mutex<std::fs::File>>);

impl Write for FileWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.lock().unwrap().flush()
    }
}

/// File logging + panic hook. Log: `{exe}/LocalHashFinder.log`
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let path = config::exe_dir().join("LocalHashFinder.log");
    let _ = LOG_PATH.set(path.clone());

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    let writer = Arc::new(Mutex::new(file));

    let file_writer = Arc::clone(&writer);
    let panic_path = path.clone();
    panic::set_hook(Box::new(move |info| {
        let msg = format!("PANIC: {info}");
        eprintln!("{msg}");
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&panic_path) {
            let _ = writeln!(f, "{msg}");
        }
        if let Ok(mut f) = file_writer.lock() {
            let _ = writeln!(f, "{msg}");
        }
    }));

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,local_hash_finder=debug"));

    let log_writer = Arc::clone(&writer);
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(move || FileWriter(Arc::clone(&log_writer)))
        .with_ansi(false)
        .init();

    tracing::info!("=== LocalHashFinder start ===");
    tracing::info!("log file: {}", path.display());
    Ok(())
}
