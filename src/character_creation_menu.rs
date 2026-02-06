use crate::{GameData, GameState, main_menu::MainMenu, playing::Playing};
use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    event::KeyCode,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::Write;

// ActorKind (race) > class? > stats > name > confirm

pub struct CharacterCreationMenu {
    // cursor: u8,
    // step: u8,

    // actor_kind: String,
}

impl CharacterCreationMenu {
    pub fn new() -> Self {
        Self {
            // cursor: 0,
            // step: 0,
        }
    }

    pub fn update(self, game_data: &mut GameData) -> GameState {
        match game_data.input.last_key() {
            KeyCode::Esc => return GameState::MainMenu(MainMenu::new()),
            KeyCode::Enter => return GameState::Playing(Playing::new()),
            _ => GameState::CharacterCreationMenu(self),
        }
    }

    pub fn draw(&self) -> std::io::Result<()> {
        let mut stdout = std::io::stdout();
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(MoveTo(0, 0))?;
        stdout.queue(Print("Character Creation"))?;
        stdout.flush()?;
        Ok(())
    }
}
