
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    input: String,
    range: Vec<[f64; 2]>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];


        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);
            // rectangle(BLACK, rectangle::square(100.0, 100.0, 100.0), c.transform, gl);
            
            // polygon(BLACK, &self.range, c.transform, gl);

            for [x, y] in self.range.iter().map(|[x, y]| [x*5.0, (y)*5.0]) {
                Rectangle::new(BLACK).draw([x, y, 2.0, 2.0], &c.draw_state, c.transform, gl);
            }
            
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.range = forust::evaluate(&self.input, 0, 100, 0, 100);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("forust", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        input: "y = x**2".to_string(),
        range: Vec::new(),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

// fn main() {
//     let input = "x**3 = y";
//     forust::evaluate(input, 0, 100, 0, 100);
// }
