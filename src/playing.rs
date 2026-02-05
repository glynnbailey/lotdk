use crate::{GameData, GameState, input::InputState, main_menu::MainMenu, position::Position};
use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    event::KeyCode,
    style::{Print, PrintStyledContent, StyledContent, Stylize, style},
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

    fn run_ai_turns(&self, game_data: &mut GameData) {
        loop {
            let actor_id = game_data.actors.next_turn().unwrap();
            if actor_id == 0 {
                return;
            }

            // dummy ai turn: wait
            self.process_action(actor_id, Action::Wait, game_data);

            // TODO implement AI logic
            // let actor = game_data.actors.get_actor(actor_id).unwrap();
            // let (actor_state, action) = actor.ai_turn(actor_id, &game_data.actors, &game_data.map);
            // game_data.actors.get_actor_mut(actor_id).unwrap().set_state(actor_state);
            // self.process_action(actor_id, action, game_data);
        }
    }

    fn process_action(&self, actor_id: usize, action: Action, game_data: &mut GameData) {
        let actor_speed = game_data.actors.get_actor(actor_id).unwrap().speed();
        let speed_modifier = (10000 / actor_speed).max(10);
        let cost = (action.cost() * speed_modifier) / 100;
        game_data.actors.end_turn(cost);

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
        // stdout.queue(Clear(ClearType::All))?;

        let player_position = game_data.actors.get_player_actor().position();
        let (width, height) = crossterm::terminal::size()?;

        for y in 0..height {
            for x in 0..width {
                let map_x = player_position.x + x as i64 - (width / 2) as i64;
                let map_y = player_position.y + y as i64 - (height / 2) as i64;

                if let Some(tile) = game_data.map.get_tile(Position { x: map_x as i64, y: map_y as i64 }) {
                    // Draw actor if present
                    if let Some(actor_id) = tile.actor_id() {
                        if let Some(actor) = game_data.actors.get_actor(actor_id) {
                            let (ch, color) = actor.glyph();
                            stdout.queue(MoveTo(x, y))?;
                            stdout.queue(PrintStyledContent(style(ch).with(color)))?;
                            continue;
                        }
                    }

                    // Draw tile
                    let (ch, color) = tile.glyph();
                    stdout.queue(MoveTo(x, y))?;
                    stdout.queue(PrintStyledContent(style(ch).with(color)))?;
                } else {
                    // draw empty space for out-of-bounds
                    stdout.queue(MoveTo(x, y))?;
                    stdout.queue(Print(' '))?;
                }
            }
        }

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
