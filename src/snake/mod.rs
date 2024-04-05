use crate::{
    shape::{square::Square, Extremities},
    types::{Direction, GameArea, Position},
};
use opengl_graphics::GlGraphics;
use piston::input::{Key, RenderArgs};

pub struct Snake {
    direction: Direction,
    squares: Vec<Square>,
}

impl Snake {
    pub fn new(pos: Position) -> Snake {
        let initial_square = Square::new(pos);
        Snake {
            direction: Direction::Right,
            squares: vec![initial_square],
        }
    }

    pub fn update(&mut self) {
        let mut first_square = self.squares.first().unwrap().clone();

        first_square.move_square(&self.direction);
        self.squares.insert(0, first_square);

        self.squares.pop();
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        gl.draw(args.viewport(), |c, gl| {
            self.squares.iter().for_each(|square| {
                let rectangle_square = rectangle::square(0.0, 0.0, square.size);
                let pos = square.get_position();
                let transform = c.transform.trans(pos.x, pos.y);
                rectangle(RED, rectangle_square, transform, gl);
            });
        });
    }

    pub fn key_pressed(&mut self, key: &Key) {
        match key {
            Key::Up => self.direction = Direction::Up,
            Key::Down => self.direction = Direction::Down,
            Key::Left => self.direction = Direction::Left,
            Key::Right => self.direction = Direction::Right,
            _ => {}
        }
    }

    pub fn intersect_wall(&self, game_area: &GameArea) -> bool {
        self.squares.first().unwrap().intersect_wall(game_area)
    }

    pub fn intersect(&self, other: &dyn Extremities) -> bool {
        self.squares.iter().any(|square| square.intersect(other))
    }

    pub fn grow(&mut self) {
        let last_square = self.squares.last().unwrap().clone();
        self.squares.push(last_square);
    }
}
