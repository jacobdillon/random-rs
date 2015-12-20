extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use graphics::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

// Color constants
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const ORANGE: [f32; 4] = [1.0, 0.498039215686, 0.0, 1.0];
const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const VIOLET: [f32; 4] = [0.56078431372, 0.0, 1.0, 1.0];

// Rainbow constant
const RAINBOW: [types::Color; 6] = [RED, ORANGE, YELLOW, GREEN, BLUE, VIOLET];

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend
    big_rotation: f64, // Rotation for the big square
    little_rotation: f64, // Rotation for the little square
    fg_color: types::Color, // Color for the big square
    bg_color: types::Color, // Color for the little square/background
    loc: (f64, f64), // Location of squares
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        // Squares
        let big_square = rectangle::square(0.0, 0.0, 80.0);
        let little_square = rectangle::square(0.0, 0.0, 40.0);

        // Assign rotation to squares
        let big_rotation = self.big_rotation;
        let little_rotation = self.little_rotation;

        // Set fg/bg_color to current fg/bg_color
        let fg_color = self.fg_color;
        let bg_color = self.bg_color;

        // Set x/y to current x/y
        let (x, y) = self.loc;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(bg_color, gl);

            // Rotate squares
            let big_transform = c.transform
                                 .trans(x, y)
                                 .rot_rad(big_rotation)
                                 .trans(-40.0, -40.0);
            let little_transform = c.transform
                                    .trans(x, y)
                                    .rot_rad(little_rotation)
                                    .trans(-20.0, -20.0);

            // Draw the squares rotating around the middle of the screen.
            rectangle(fg_color, big_square, big_transform, gl);
            rectangle(bg_color, little_square, little_transform, gl)
        });
    }

    fn update_color(&mut self, indexes: (usize, usize), up_or_down: i32) -> (usize, usize) {
        // Set new indexes to old indexes
        let mut fg_index = indexes.0;
        let mut bg_index = indexes.1;

        if up_or_down == 1 {
            // Cant go above 5 or below 0
            if fg_index != 5 && bg_index != 0 {
                // Update index
                fg_index += 1;
                bg_index -= 1;
            }
        } else {
            // Cant go below 0 or above 5
            if fg_index != 0 && bg_index != 5 {
                // Update index
                fg_index -= 1;
                bg_index += 1;
            }
        }

        // Update color
        self.fg_color = RAINBOW[fg_index];
        self.bg_color = RAINBOW[bg_index];

        // Return new tuple containing updated fg_index and bg_index
        (fg_index, bg_index)
    }

    fn update_rotation(&mut self, args: &UpdateArgs, direction: f64) {
        // Rotate 2 radians per second
        self.big_rotation += direction * (2.0 * args.dt);
        self.little_rotation -= direction * (2.0 * args.dt);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working
    let opengl = OpenGL::V3_2;

    let mut direction: f64 = 1.0;
    let mut indexes: (usize, usize) = (5, 0);

    // Create an Glutin window
    let window: Window = WindowSettings::new("spinning", [200, 200])
                             .opengl(opengl)
                             .exit_on_esc(true)
                             .build()
                             .unwrap();

    // Create a new App and run it
    let mut app = App {
        gl: GlGraphics::new(opengl),
        big_rotation: 0.0,
        little_rotation: 0.0,
        fg_color: RAINBOW[indexes.0],
        bg_color: RAINBOW[indexes.1],
        loc: (100.0, 100.0),
    };

    for e in window.events() {
        // Render app
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        // Update rotation
        if let Some(u) = e.update_args() {
            app.update_rotation(&u, direction);
        }

        // Get button press
        if let Some(button) = e.press_args() {
            match button {
                // Change direction
                Button::Keyboard(Key::Right) => direction = 1.0,
                Button::Keyboard(Key::Left) => direction = -1.0,

                // Move rectangles
                Button::Keyboard(Key::W) => app.loc.1 -= 5.0,
                Button::Keyboard(Key::S) => app.loc.1 += 5.0,
                Button::Keyboard(Key::A) => app.loc.0 -= 5.0,
                Button::Keyboard(Key::D) => app.loc.0 += 5.0,

                // Reset position
                Button::Keyboard(Key::Space) => app.loc = (100.0, 100.0),

                // Change color
                Button::Keyboard(Key::Up) => indexes = app.update_color(indexes, 1),
                Button::Keyboard(Key::Down) => indexes = app.update_color(indexes, 0),

                // Default
                _ => (),
            }
        }
    }
}
