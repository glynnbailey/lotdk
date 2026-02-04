use crate::position::Position;

pub struct Actor {
    position: Position,
}

impl Actor {
    pub fn new(position: Position) -> Self {
        Self { position }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }
}
