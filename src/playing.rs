use crate::{GameData, GameState, input::InputState, main_menu::MainMenu};
use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    event::KeyCode,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::Write;

#[derive(Clone, Copy)]
pub struct Playing;

impl Playing {
    pub fn update(self, game_data: &mut GameData) -> GameState {
        if game_data.input.is_pressed(KeyCode::Esc) {
            return GameState::MainMenu(MainMenu::new());
        }

        GameState::Playing(self)
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
