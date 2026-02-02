#![allow(dead_code, unused)]

mod input;
mod main_menu;
mod playing;
mod terminalguard;

pub use main_menu::MainMenu;
pub use playing::Playing;

use crate::input::InputState;
use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    event::KeyCode,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::Write;

pub enum GameState {
    MainMenu(main_menu::MainMenu),
    Playing(playing::Playing),
    Quit,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Quit
    }
}

pub struct GameData {
    pub input: InputState,
}

impl GameData {
    pub fn new() -> Self {
        GameData { input: InputState::new() }
    }
}

fn main() -> std::io::Result<()> {
    let _term = terminalguard::TerminalGuard::new()?;
    let mut game_data = GameData::new();
    let mut game_state = GameState::MainMenu(main_menu::MainMenu);

    loop {
        // draw
        match game_state {
            GameState::MainMenu(main_menu) => main_menu.draw(&game_data)?,
            GameState::Playing(playing) => playing.draw(&game_data)?,
            GameState::Quit => break,
        }

        // input
        game_data.input.update()?;

        // update
        game_state = match game_state {
            GameState::MainMenu(main_menu) => main_menu.update(&mut game_data),
            GameState::Playing(playing) => playing.update(&mut game_data),
            GameState::Quit => GameState::Quit,
        };
    }

    Ok(())
}
