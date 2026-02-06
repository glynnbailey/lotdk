use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::io::Write;
use std::panic;

pub struct TerminalGuard;

impl TerminalGuard {
    pub fn new() -> std::io::Result<Self> {
        // let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            Self::cleanup();
            eprintln!("{}", panic_info);
            std::process::exit(101);
        }));

        enable_raw_mode()?;
        execute!(std::io::stdout(), EnterAlternateScreen, Hide)?;
        Ok(Self)
    }

    fn cleanup() {
        let mut stdout = std::io::stdout();
        let _ = disable_raw_mode();
        let _ = execute!(stdout, LeaveAlternateScreen, Show);
        let _ = stdout.flush();
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        Self::cleanup();
    }
}
