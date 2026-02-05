#![allow(dead_code, unused)]

mod actor;
mod actor_manager;
mod consts;
mod input;
mod main_menu;
mod map_manager;
mod playing;
mod position;
mod shadowcast;
mod terminalguard;
mod debug;

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
    pub input: input::InputState,
    pub actors: actor_manager::ActorManager,
    pub map: map_manager::MapManager,
}

impl GameData {
    pub fn new() -> Self {
        GameData {
            input: input::InputState::new(),
            actors: actor_manager::ActorManager::new(),
            map: map_manager::MapManager::new(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let _term = terminalguard::TerminalGuard::new()?;
    let mut game_data = GameData::new();
    let mut game_state = GameState::MainMenu(main_menu::MainMenu::new());

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
