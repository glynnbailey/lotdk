use crossterm::style::Color;

use crate::{actor_manager::ActorManager, consts::DUNGEON_SIZE, consts::*, position::Position};

pub struct MapManager {
    tiles: Vec<Tile>,
}

impl MapManager {
    pub fn new() -> Self {
        Self { tiles: Vec::with_capacity(DUNGEON_SIZE * DUNGEON_SIZE) }
    }

    pub fn build_floor(&mut self) {
        self.tiles.clear();
        // let layout = tunneller::generate_tunneller_layout();
        // for y in 0..DUNGEON_SIZE {
        //     for x in 0..DUNGEON_SIZE {
        //         self.tiles.push(Tile::new(layout[y * DUNGEON_SIZE + x]));
        //     }
        // }

        for _ in 0..(DUNGEON_SIZE * DUNGEON_SIZE) {
            self.tiles.push(Tile::new(TileType::Floor));
        }

        self.tiles[5 * DUNGEON_SIZE + 5] = Tile::new(TileType::Wall);
        self.tiles[5 * DUNGEON_SIZE + 6] = Tile::new(TileType::Wall);
        self.tiles[5 * DUNGEON_SIZE + 7] = Tile::new(TileType::ClosedDoor);
        self.tiles[5 * DUNGEON_SIZE + 8] = Tile::new(TileType::Wall);
        self.tiles[5 * DUNGEON_SIZE + 9] = Tile::new(TileType::OpenDoor);
        self.tiles[5 * DUNGEON_SIZE + 10] = Tile::new(TileType::Wall);
        self.tiles[5 * DUNGEON_SIZE + 11] = Tile::new(TileType::Wall);
    }

    pub fn get_tile(&self, position: Position) -> Option<&Tile> {
        if position.x < 0 || position.x >= DUNGEON_SIZE as i64 || position.y < 0 || position.y >= DUNGEON_SIZE as i64 {
            return None;
        }
        let index = (position.y as usize) * DUNGEON_SIZE + (position.x as usize);
        self.tiles.get(index)
    }

    pub fn get_tile_mut(&mut self, position: Position) -> Option<&mut Tile> {
        if position.x < 0 || position.x >= DUNGEON_SIZE as i64 || position.y < 0 || position.y >= DUNGEON_SIZE as i64 {
            return None;
        }
        let index = (position.y as usize) * DUNGEON_SIZE + (position.x as usize);
        self.tiles.get_mut(index)
    }

    /// Useful for placing actors/items randomly
    pub fn get_unoccupied_floor_tiles(&self) -> Vec<Position> {
        let mut floor_positions = Vec::new();
        for y in 0..DUNGEON_SIZE {
            for x in 0..DUNGEON_SIZE {
                let index = y * DUNGEON_SIZE + x;
                if let TileType::Floor = self.tiles[index].tile_type
                    && self.tiles[index].actor_id.is_none()
                {
                    floor_positions.push(Position { x: x as i64, y: y as i64 });
                }
            }
        }
        floor_positions
    }

    pub fn update_visibility(&mut self, position: Position) {
        // Set all visible tiles to explored
        for tile in &mut self.tiles {
            if let Visibility::Visible = tile.visibility {
                tile.visibility = Visibility::Explored;
            }
        }

        // Run shadowcast from the given position
        for tile_pos in self.shadowcast(position) {
            self.tiles[(tile_pos.y as usize) * DUNGEON_SIZE + (tile_pos.x as usize)].visibility = Visibility::Visible;
        }
    }

    pub fn set_actor(&mut self, position: Position, actor_id: usize) {
        if let Some(tile) = self.tiles.get_mut((position.y as usize) * DUNGEON_SIZE + (position.x as usize)) {
            tile.actor_id = Some(actor_id);
        }
    }

    pub fn move_actor(&mut self, from: Position, to: Position) {
        let from_index = (from.y as usize) * DUNGEON_SIZE + (from.x as usize);
        let to_index = (to.y as usize) * DUNGEON_SIZE + (to.x as usize);
        assert!(self.tiles[from_index].actor_id.is_some(), "No actor at the 'from' position");
        assert!(self.tiles[to_index].actor_id.is_none(), "Destination 'to' position already occupied");
        let actor_id = self.tiles[from_index].actor_id.take();
        self.tiles[to_index].actor_id = actor_id;
    }

    pub fn remove_actor(&mut self, position: Position) {
        self.tiles[(position.y as usize) * DUNGEON_SIZE + (position.x as usize)].actor_id.take();
    }
}

pub struct Tile {
    tile_type: TileType,
    visibility: Visibility,
    actor_id: Option<usize>,
    // items: Inventory,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Self {
            tile_type,
            visibility: Visibility::Hidden,
            actor_id: None,
            // items: Inventory::new(),
        }
    }

    pub fn tile_type(&self) -> TileType {
        self.tile_type
    }

    pub fn actor_id(&self) -> Option<usize> {
        self.actor_id
    }

    pub fn glyph(&self) -> (char, Color) {
        match self.visibility {
            Visibility::Hidden => (' ', Color::Black),
            Visibility::Visible => match self.tile_type {
                TileType::Floor => ('.', Color::Grey),
                TileType::Wall => ('#', Color::Grey),
                TileType::ClosedDoor => ('+', Color::Yellow),
                TileType::OpenDoor => ('-', Color::Yellow),
            },
            Visibility::Explored => match self.tile_type {
                TileType::Floor => ('.', Color::DarkGrey),
                TileType::Wall => ('#', Color::DarkGrey),
                TileType::ClosedDoor => ('+', Color::DarkYellow),
                TileType::OpenDoor => ('-', Color::DarkYellow),
            },
        }
    }

    pub fn blocks_vision(&self) -> bool {
        match self.tile_type {
            TileType::Wall => true,
            TileType::Floor => false,
            TileType::ClosedDoor => true,
            TileType::OpenDoor => false,
        }
    }

    pub fn movement_cost(&self) -> u32 {
        match self.tile_type {
            TileType::Wall => u32::MAX,
            TileType::Floor => 1,
            TileType::ClosedDoor => 5,
            TileType::OpenDoor => 1,
        }
    }

    pub fn interact(&mut self) {
        match self.tile_type {
            TileType::Wall => unreachable!(),
            TileType::Floor => unreachable!(),
            TileType::ClosedDoor => self.tile_type = TileType::OpenDoor,
            TileType::OpenDoor => self.tile_type = TileType::ClosedDoor,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Floor,
    Wall,
    ClosedDoor,
    OpenDoor,
}

#[derive(Clone, Copy)]
pub enum Visibility {
    Hidden,
    Visible,
    Explored,
}
