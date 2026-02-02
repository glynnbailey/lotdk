use crate::{GameData, GameState, input::InputState, playing::Playing};
use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    event::KeyCode,
    style::{Attribute, Color, Print, PrintStyledContent, StyledContent, Stylize, style},
    terminal::{Clear, ClearType},
};
use std::io::Write;

const MENU_ITEMS: [&str; 2] = ["Start Game", "Quit"];

#[derive(Clone, Copy)]
pub struct MainMenu {
    cursor: u8,
}

impl MainMenu {
    pub fn new() -> Self {
        Self { cursor: 0 }
    }

    pub fn update(mut self, game_data: &mut GameData) -> GameState {
        if game_data.input.is_pressed(KeyCode::Char('w')) || game_data.input.is_pressed(KeyCode::Up) {
            self.cursor = if self.cursor > 0 { self.cursor - 1 } else { (MENU_ITEMS.len() - 1) as u8 };
        }

        if game_data.input.is_pressed(KeyCode::Char('s')) || game_data.input.is_pressed(KeyCode::Down) {
            self.cursor = if (self.cursor as usize) < MENU_ITEMS.len() - 1 { self.cursor + 1 } else { 0 };
        }

        if game_data.input.is_pressed(KeyCode::Enter) {
            match self.cursor {
                0 => return GameState::Playing(Playing),
                1 => return GameState::Quit,
                _ => {}
            }
        }

        if game_data.input.is_pressed(KeyCode::Esc) {
            return GameState::Quit;
        }

        GameState::MainMenu(self)
    }

    pub fn draw(&self, game_data: &GameData) -> std::io::Result<()> {
        let mut stdout = std::io::stdout();
        stdout.queue(Clear(ClearType::All))?;

        stdout.queue(MoveTo(0, 0))?;
        stdout.queue(PrintStyledContent(style("Lair of the Daemon King").with(Color::Magenta).attribute(Attribute::Bold)))?;

        stdout.queue(MoveTo(0, 3))?;
        stdout.queue(Print("Main Menu"))?;

        for (i, item) in MENU_ITEMS.iter().enumerate() {
            stdout.queue(MoveTo(0, 5 + i as u16))?;
            if i as u8 == self.cursor {
                stdout.queue(PrintStyledContent(style(item).with(Color::Yellow)))?;
            } else {
                stdout.queue(Print(format!("{}", item)))?;
            }
        }

        stdout.flush()?;
        Ok(())
    }
}
