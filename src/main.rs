#![allow(dead_code, unused)]

mod input;
mod terminalguard;

use std::io::Write;

use crate::input::InputState;
use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    event::KeyCode,
    style::Print,
    terminal::{Clear, ClearType},
};

enum GameState {
    MainMenu(MainMenu),
    Playing(Playing),
    Quit,
}

#[derive(Clone, Copy)]
struct MainMenu;

impl MainMenu {
    pub fn update(&self, game_data: &mut GameData) -> GameState {
        if game_data.input.last_key() == KeyCode::Enter {
            return GameState::Playing(Playing);
        }

        if game_data.input.last_key() == KeyCode::Esc {
            return GameState::Quit;
        }

        GameState::MainMenu(MainMenu)
    }

    pub fn draw(&self, game_data: &GameData) -> std::io::Result<()> {
        let mut stdout = std::io::stdout();
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(MoveTo(0, 0))?;
        stdout.queue(Print("Main Menu"))?;
        stdout.flush()?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct Playing;

impl Playing {
    pub fn update(&self, game_data: &mut GameData) -> GameState {
        if game_data.input.last_key() == KeyCode::Esc {
            return GameState::MainMenu(MainMenu);
        }

        GameState::Playing(Playing)
    }

    pub fn draw(&self, game_data: &GameData) -> std::io::Result<()> {
        let mut stdout = std::io::stdout();
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(MoveTo(0, 0))?;
        stdout.queue(Print("Playing"))?;
        stdout.flush()?;
        Ok(())
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
    let mut game_state = GameState::MainMenu(MainMenu);

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
            GameState::MainMenu(ref main_menu) => main_menu.update(&mut game_data),
            GameState::Playing(ref playing) => playing.update(&mut game_data),
            GameState::Quit => break,
        };
    }

    Ok(())
}
