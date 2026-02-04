use crate::{map_manager::MapManager, position::Position};

const VISION_RADIUS: i64 = 20;

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Access {
    Unknown,
    Impassible,
    Passible,
}

impl MapManager {
    /// Takes an origin position and returns a Vec<Position> of tiles that are visible from the origin.
    pub fn shadowcast(&self, origin: Position) -> Vec<Position> {
        let mut visible_tiles: Vec<Position> = Vec::new();

        // Origin is always visible
        visible_tiles.push(origin.clone());

        for direction in [Direction::North, Direction::South, Direction::East, Direction::West] {
            let first_row = Row::new(1, -1.0, 1.0);
            self.scan(first_row, origin, direction, &mut visible_tiles);
        }

        visible_tiles
    }

    fn scan(&self, mut row: Row, origin: Position, direction: Direction, visible_tiles: &mut Vec<Position>) {
        if row.depth > VISION_RADIUS {
            return;
        }

        let mut prev_tile_access = Access::Unknown;
        for col in row.cols() {
            // get the tile position
            let tile_position = transform_quadrant(&(origin.x, origin.y), &direction, row.depth, col);

            // check if the position is impassible
            let mut tile_access = Access::Passible;
            match self.get_tile(tile_position) {
                Some(ref tile) => {
                    if tile.blocks_vision() {
                        tile_access = Access::Impassible;
                    }
                }
                None => continue,
            }

            // Add visible tiles - all tiles in vision should be visible if not blocked
            if is_symmetric(&row, col) {
                visible_tiles.push(tile_position);
            }

            // Handle shadow transitions
            if prev_tile_access == Access::Impassible && tile_access == Access::Passible {
                row.start_slope = slope(row.depth, col);
            }

            // update end slope and scan
            if prev_tile_access == Access::Passible && tile_access == Access::Impassible {
                let mut next_row = row.next();
                next_row.end_slope = slope(row.depth, col);
                self.scan(next_row, origin, direction, visible_tiles);
            }

            prev_tile_access = tile_access
        }

        if prev_tile_access == Access::Passible {
            self.scan(row.next(), origin, direction, visible_tiles);
        }
    }
}

fn transform_quadrant(origin: &(i64, i64), direction: &Direction, depth: i64, col: i64) -> Position {
    match direction {
        Direction::North => Position { x: origin.0 + col, y: origin.1 + depth },
        Direction::South => Position { x: origin.0 + col, y: origin.1 - depth },
        Direction::East => Position { x: origin.0 + depth, y: origin.1 + col },
        Direction::West => Position { x: origin.0 - depth, y: origin.1 + col },
    }
}

struct Row {
    depth: i64,
    start_slope: f32,
    end_slope: f32,
}

impl Row {
    fn new(depth: i64, start_slope: f32, end_slope: f32) -> Self {
        Self { depth, start_slope, end_slope }
    }

    fn next(&self) -> Self {
        Self {
            depth: self.depth + 1,
            start_slope: self.start_slope,
            end_slope: self.end_slope,
        }
    }

    fn cols(&self) -> Vec<i64> {
        let min = ((self.depth as f32 * self.start_slope) + 0.5).floor() as i64;
        let max = ((self.depth as f32 * self.end_slope) - 0.5).ceil() as i64;
        let mut cols = Vec::new();
        for i in min..=max {
            cols.push(i);
        }
        cols
    }
}

// Consider using a more precise slope calculation for better symmetry
fn slope(depth: i64, col: i64) -> f32 {
    (2.0 * col as f32 - 1.0) / (2.0 * depth as f32)
}

fn is_symmetric(row: &Row, col: i64) -> bool {
    col as f32 >= row.depth as f32 * row.start_slope && col as f32 <= row.depth as f32 * row.end_slope
}
