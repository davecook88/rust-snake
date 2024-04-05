extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod shape;
mod snake;
pub mod types;

use crate::types::Position;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::EventLoop;

const WINDOW_SIZE: [f64; 2] = [200.0, 200.0];

enum GameState {
    Playing,
    GameOver,
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    snake: snake::Snake,
    game_area: types::GameArea,
    state: GameState,
    glyphs: GlyphCache<'static>,
}

impl App {
    fn new(gl: GlGraphics) -> App {
        let (x, y) = (WINDOW_SIZE[0] / 2.0, WINDOW_SIZE[1] / 2.0);

        let snake = snake::Snake::new(Position { x, y });
        let glyphs = GlyphCache::new("src/fonts/Roboto-Black.ttf", (), TextureSettings::new())
            .expect("Could not load font");
        App {
            gl,
            snake,
            game_area: types::GameArea {
                min_x: 0.0,
                max_x: WINDOW_SIZE[0],
                min_y: 0.0,
                max_y: WINDOW_SIZE[1],
            },
            state: GameState::Playing,
            glyphs,
        }
    }

    fn key_pressed(&mut self, key: &Key) {
        self.snake.key_pressed(key);
    }
    fn render(&mut self, args: &RenderArgs) {
        match self.state {
            GameState::Playing => self.render_playing(args),
            GameState::GameOver => self.render_game_over(args),
        }
    }

    fn render_playing(&mut self, args: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        self.gl.draw(args.viewport(), |_, gl| {
            // Clear the screen.
            graphics::clear(GREEN, gl);
        });

        self.snake.render(&mut self.gl, args);
    }

    fn render_game_over(&mut self, args: &RenderArgs) {
        // TODO: Add a struct for rendering text, which can also calculate the size
        // based on the glyphs
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let gl = &mut self.gl;
        let glyphs = &mut self.glyphs;

        let text = "GAME OVER";
        let text_size = 24;
        let text_height = text_size as f64;

        let x = text_size as f64;
        let y = (WINDOW_SIZE[1] + text_height) / 2.0;

        gl.draw(args.viewport(), |c, gl| {
            // Assuming 'glyphs' is accessible here, and 'self.gl' is your GlGraphics instance
            let transform = c.transform.trans(x, y); // Example position
            text::Text::new_color(RED, text_size)
                .draw(text, glyphs, &c.draw_state, transform, gl)
                .unwrap();
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // update every 0.2 seconds
        println!("{}", args.dt);

        match self.state {
            GameState::Playing => self.update_playing(),
            GameState::GameOver => self.update_game_over(args),
        }
    }

    fn update_playing(&mut self) {
        self.snake.update();
        if self.snake.intersect_wall(&self.game_area) {
            self.state = GameState::GameOver;
        }
    }

    fn update_game_over(&mut self, _args: &UpdateArgs) {
        // Do nothing
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", WINDOW_SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new().ups(10));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.key_pressed(&key);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
