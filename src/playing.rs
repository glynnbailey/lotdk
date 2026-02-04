use crate::{GameData, GameState, input::InputState, main_menu::MainMenu, position::Position};
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
        // State changes
        match game_data.input.last_key() {
            KeyCode::Esc => return GameState::MainMenu(MainMenu::new()),
            _ => {}
        }

        // Turn handling
        if let Some(action) = self.handle_player_turn(game_data) {
            self.process_action(0, action, game_data);
            self.run_ai_turns(game_data);
            game_data.map.update_visibility(game_data.actors.get_player_actor().position());
        }

        GameState::Playing(self)
    }

    fn handle_player_turn(&self, game_data: &mut GameData) -> Option<Action> {
        match game_data.input.last_key() {
            KeyCode::Char('8') => Some(Action::MoveTo(Position { x: 0, y: -1 } + game_data.actors.get_player_actor().position())),
            KeyCode::Char('9') => Some(Action::MoveTo(Position { x: 1, y: -1 } + game_data.actors.get_player_actor().position())),
            KeyCode::Char('6') => Some(Action::MoveTo(Position { x: 1, y: 0 } + game_data.actors.get_player_actor().position())),
            KeyCode::Char('3') => Some(Action::MoveTo(Position { x: 1, y: 1 } + game_data.actors.get_player_actor().position())),
            KeyCode::Char('2') => Some(Action::MoveTo(Position { x: 0, y: 1 } + game_data.actors.get_player_actor().position())),
            KeyCode::Char('1') => Some(Action::MoveTo(Position { x: -1, y: 1 } + game_data.actors.get_player_actor().position())),
            KeyCode::Char('4') => Some(Action::MoveTo(Position { x: -1, y: 0 } + game_data.actors.get_player_actor().position())),
            KeyCode::Char('7') => Some(Action::MoveTo(Position { x: -1, y: -1 } + game_data.actors.get_player_actor().position())),
            _ => None,
        }
    }

    fn run_ai_turns(&self, game_data: &mut GameData) {}

    fn process_action(&self, actor_id: usize, action: Action, game_data: &mut GameData) {
        match action {
            Action::Wait => {
                // Do nothing
            }
            Action::MoveTo(destination_position) => {
                if let Some(actor) = game_data.actors.get_actor_mut(actor_id) {
                    let actor = game_data.actors.get_actor_mut(actor_id).unwrap();
                    let current_position = actor.position();
                    game_data.map.move_actor(current_position, destination_position);
                    actor.set_position(destination_position);
                }
            }
            Action::Interact(target_pos) => {
                // Interaction logic here
            }
            Action::MeleeAttack(target_id) => {
                // Melee attack logic here
            }
        }
    }

    pub fn draw(&self, game_data: &GameData) -> std::io::Result<()> {
        let mut stdout = std::io::stdout();
        stdout.queue(Clear(ClearType::All))?;
        let player_position = game_data.actors.get_player_actor().position();
        stdout.queue(MoveTo(player_position.x as u16, player_position.y as u16))?;
        stdout.queue(Print('@'))?;
        stdout.flush()?;
        Ok(())
    }
}

pub enum Action {
    Wait,
    MoveTo(Position),
    Interact(Position),
    MeleeAttack(usize),
}

impl Action {
    fn cost(&self) -> u32 {
        match self {
            Action::Wait => 100,
            Action::MoveTo(_) => 100,
            Action::Interact(_) => 100,
            Action::MeleeAttack(_) => 100,
        }
    }
}
