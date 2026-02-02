use crate::{GameData, GameState, input::InputState, playing::Playing};
use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    event::KeyCode,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::Write;

#[derive(Clone, Copy)]
pub struct MainMenu;

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
