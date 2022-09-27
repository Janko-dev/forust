
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
    // input: String,
    range: Vec<[f64; 2]>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const LINE: [f32; 4] = [0.7, 0.2, 0.2, 1.0];
        const BG: [f32; 4] = [0.2, 0.8, 0.85, 0.6];

        let [w, h] = args.window_size;

        self.gl.draw(args.viewport(), |c, gl| {
            
            let c = c.trans(w/2.0, h/2.0);

            clear(BG, gl);
            
            Line::new(BLACK, 1.0).draw_from_to(
                [-w/2.0, 0.0], 
                [w, 0.0], 
                &c.draw_state, c.transform, gl);
            
            Line::new(BLACK, 1.0).draw_from_to(
                [0.0, -h/2.0], 
                [0.0, h], 
                &c.draw_state, c.transform, gl);
            
            for i in 0..self.range.len() {
                let curr = self.range.get(i).unwrap();
                let next = self.range.get(i+1).unwrap_or(curr);
                Line::new(LINE, 1.0)
                    .draw_from_to(*curr, *next, &c.draw_state, c.transform, gl);

                // let [x, y] = self.range.get(i).unwrap();
                // rectangle(LINE, [*x, *y, 2.0, 2.0], c.transform, gl);
            }
            
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        // self.range = forust::evaluate(&self.input, 0, 100, 0, 100);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("forust", [600, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let input = "y = x**3";
    let dims = (-600, 600);
    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        range: forust::evaluate(input, dims, dims)
            .iter()
            .map(|[x, y]| [*x, -*y])
            .collect(),
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