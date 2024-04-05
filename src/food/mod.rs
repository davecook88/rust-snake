use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use crate::shape::{self};

pub struct Food {
    pub square: shape::square::Square,
}

impl Food {
    pub fn new(pos: crate::types::Position) -> Food {
        Food {
            square: shape::square::Square::new(pos),
        }
    }

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let squares = rectangle::square(0.0, 0.0, self.square.size);

        gl.draw(args.viewport(), |c, gl| {
            let pos = self.square.get_position();
            let transform = c.transform.trans(pos.x, pos.y);
            rectangle(WHITE, squares, transform, gl);
        });
    }
}
