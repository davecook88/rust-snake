pub mod square;

use crate::types;
pub trait HasPosition {
    fn position(&self) -> &types::Position;
}

pub trait HasSize {
    fn size(&self) -> f64;
}

pub trait Extremities: HasPosition + HasSize {
    fn top(&self) -> f64 {
        self.position().y - self.size() / 2.0
    }
    fn bottom(&self) -> f64 {
        self.position().y + self.size() / 2.0
    }
    fn left(&self) -> f64 {
        self.position().x - self.size() / 2.0
    }
    fn right(&self) -> f64 {
        self.position().x + self.size() / 2.0
    }

    // Checks if the object intersects with another object
    fn intersect(&self, other: &dyn Extremities) -> bool {
        self.right() >= other.left()
            && self.left() <= other.right()
            && self.bottom() >= other.top()
            && self.top() <= other.bottom()
    }

    fn intersect_wall(&self, game_area: &types::GameArea) -> bool {
        self.right() >= game_area.max_x
            || self.left() <= game_area.min_x
            || self.bottom() <= game_area.min_y
            || self.top() >= game_area.max_y
    }
}
