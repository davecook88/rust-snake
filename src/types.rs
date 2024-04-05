#[derive(Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct GameArea {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}
