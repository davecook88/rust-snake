use crate::{
    shape::{square::Square, Extremities},
    types::{Direction, GameArea, Position},
};
use opengl_graphics::GlGraphics;
use piston::input::{Key, RenderArgs};

pub struct Snake {
    direction: Direction,
    size: f64,
    squares: Vec<Square>,
}

impl Snake {
    pub fn new(pos: Position) -> Snake {
        let initial_square = Square::new(pos);
        Snake {
            direction: Direction::Right,
            size: 5.0,
            squares: vec![initial_square],
        }
    }

    pub fn update(&mut self) {
        // only update every 0.2 seconds

        // print the position of the first square
        let mut first_square = self.squares.first().unwrap().clone();
        println!("Before move{:?}", first_square.get_position());

        first_square.move_square(&self.direction);
        println!("After move{:?}", first_square.get_position());
        self.squares.insert(0, first_square);

        self.squares.pop();

        // print the position of the first square
        println!("{:?}", self.squares.first().unwrap().get_position());
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares = rectangle::square(0.0, 0.0, self.size);

        gl.draw(args.viewport(), |c, gl| {
            self.squares.iter().for_each(|square| {
                let pos = square.get_position();
                let transform = c.transform.trans(pos.x, pos.y);
                rectangle(RED, squares, transform, gl);
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
}
