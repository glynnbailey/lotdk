use crate::{GameData, GameState, actor::Actor, actor_manager::ActorManager, input::InputState, map_manager::MapManager, playing::Playing, position::Position};
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
        match game_data.input.last_key() {
            KeyCode::Char('w') | KeyCode::Up => {
                self.cursor = if self.cursor > 0 { self.cursor - 1 } else { (MENU_ITEMS.len() - 1) as u8 };
            }
            KeyCode::Char('s') | KeyCode::Down => {
                self.cursor = if (self.cursor as usize) < MENU_ITEMS.len() - 1 { self.cursor + 1 } else { 0 };
            }
            KeyCode::Enter => match self.cursor {
                0 => {
                    // setup new game
                    game_data.actors = ActorManager::new();
                    game_data.map = MapManager::new();
                    game_data.map.build_floor();

                    let player_position = Position { x: 10, y: 10 };
                    let player_actor = Actor::new(player_position);
                    let actor_id = game_data.actors.add_actor(player_actor);
                    game_data.map.set_actor(player_position, actor_id);

                    game_data.map.update_visibility(player_position);

                    return GameState::Playing(Playing);
                }
                1 => return GameState::Quit,
                _ => {}
            },
            KeyCode::Esc => return GameState::Quit,
            _ => {}
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
