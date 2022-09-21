
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

// use std::convert::TryInto;

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
        const WHITE: [f32; 4] = [0.4, 0.8, 1.0, 0.9];

        self.gl.draw(args.viewport(), |c, gl| {
            
            clear(WHITE, gl);
            
            
            for i in 0..self.range.len() {
                let curr = self.range.get(i).unwrap();
                let next = self.range.get(i+1).unwrap_or(curr);
                Line::new(BLACK, 2.0).draw_from_to(*curr, *next, &c.draw_state, c.transform, gl);


                // let mut curr = self.range.get(i)
                //         .unwrap_or(&[0.0, 0.0])
                //         .to_vec();
                // curr.push(4.0);
                // curr.push(4.0);
                // let curr = curr.try_into().unwrap_or([0.0, 0.0, 0.0, 0.0]);
                //         // .try_into()
                //         // .unwrap_or_else([0.0, 0.0, 0.0, 0.0]);
                // Rectangle::new(BLACK).draw(curr, &c.draw_state, c.transform, gl);
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
    let mut window: Window = WindowSettings::new("forust", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let input = "y = (x**2)**0.5";
    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        range: forust::evaluate(input, -800, 800, -800, 800)
            .iter()
            .map(|[x, y]| [*x, 800.0 - *y])
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