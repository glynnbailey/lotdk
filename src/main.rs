// #![allow(dead_code)]

mod actor;
mod actor_manager;
mod assets;
mod character_creation_menu;
mod consts;
mod debug;
mod input;
mod inventory;
mod main_menu;
mod map_manager;
mod pathfinding;
mod playing;
mod position;
mod shadowcast;
mod terminalguard;

pub enum GameState {
    MainMenu(main_menu::MainMenu),
    CharacterCreationMenu(character_creation_menu::CharacterCreationMenu),
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
    let mut game_state = Some(GameState::MainMenu(main_menu::MainMenu::new()));

    loop {
        // draw
        match game_state.as_ref().unwrap() {
            GameState::MainMenu(main_menu) => main_menu.draw()?,
            GameState::CharacterCreationMenu(character_creation_menu) => character_creation_menu.draw()?,
            GameState::Playing(playing) => playing.draw(&game_data)?,
            GameState::Quit => break,
        }

        // input
        game_data.input.update()?;

        // update
        game_state = Some(match game_state.take().unwrap() {
            GameState::MainMenu(main_menu) => main_menu.update(&mut game_data),
            GameState::CharacterCreationMenu(character_creation_menu) => character_creation_menu.update(&mut game_data),
            GameState::Playing(playing) => playing.update(&mut game_data),
            GameState::Quit => GameState::Quit,
        });
    }

    Ok(())
}
