#![allow(dead_code)]

mod game;
mod terminalguard;

fn main() -> std::io::Result<()> {
    let _term = terminalguard::TerminalGuard::new()?;
    game::Game::new().run()?;
    Ok(())
}
