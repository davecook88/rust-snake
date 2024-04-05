use std::clone;

use crate::{
    constants,
    shape::{Extremities, HasPosition, HasSize},
    types::{Direction, Position},
};

pub struct Square {
    position: Position,
    pub size: f64,
}

impl HasPosition for Square {
    fn position(&self) -> &Position {
        &self.position
    }
}

impl HasSize for Square {
    fn size(&self) -> f64 {
        self.size
    }
}

impl Extremities for Square {}

impl Square {
    pub fn new(pos: Position) -> Square {
        Square {
            position: pos,
            size: constants::SQUARE_SIZE,
        }
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn move_square(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.position.y -= self.size,
            Direction::Down => self.position.y += self.size,
            Direction::Left => self.position.x -= self.size,
            Direction::Right => self.position.x += self.size,
        }
    }
}

impl clone::Clone for Square {
    fn clone(&self) -> Self {
        Square {
            position: Position {
                x: self.position.x,
                y: self.position.y,
            },
            size: self.size,
        }
    }
}
