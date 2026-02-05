use std::fs::OpenOptions;
use std::io::Write;

/// Provides a simple logging mechanism for debugging purposes. Logs messages to a file named "debug.log".
pub fn log_debug(message: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("debug.log")
    {
        let _ = writeln!(file, "{}", message);
    }
}
