#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn octile_distance(&self, other: Position) -> f32 {
        let dx = (self.x - other.x).abs() as f32;
        let dy = (self.y - other.y).abs() as f32;
        let f = 2f32.sqrt() - 1.0;
        if dx < dy { f * dx + dy } else { f * dy + dx }
    }

    pub fn get_neighbours(&self) -> Vec<(Position, f32)> {
        vec![
            (Position { x: self.x - 1, y: self.y }, 1.0),
            (Position { x: self.x + 1, y: self.y }, 1.0),
            (Position { x: self.x, y: self.y - 1 }, 1.0),
            (Position { x: self.x, y: self.y + 1 }, 1.0),
            (Position { x: self.x - 1, y: self.y - 1 }, 1.414),
            (Position { x: self.x + 1, y: self.y - 1 }, 1.414),
            (Position { x: self.x - 1, y: self.y + 1 }, 1.414),
            (Position { x: self.x + 1, y: self.y + 1 }, 1.414),
        ]
    }

    pub fn is_adjacent(&self, other: Position) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1 && *self != other
    }
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Position {
        Position { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
