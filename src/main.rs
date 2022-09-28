
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::{TextEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    range: Vec<[f64; 2]>,
    input: String
}

impl App {

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const LINE: [f32; 4] = [0.7, 0.2, 0.2, 1.0];
        const BG: [f32; 4] = [0.2, 0.8, 0.85, 0.6];

        let mut glyph_cache = GlyphCache::new("comic.ttf", (), TextureSettings::new()).unwrap();

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

            text(BLACK, 26, &self.input, &mut glyph_cache, c.transform.trans(0.0, -h/2.0 + 30.0), gl).unwrap();
            
        });

        
    }

    fn update(&mut self, _args: &UpdateArgs) {
        
    }

    fn handle_keys(&mut self, args: &str) {
        println!("PRESSED: '{}'", args);
        self.input.push_str(args);
        // match args {
        //     "\n" => {
        //         // fire
        //     },
        //     ""
        // }
    }
}

fn map_val(n: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    (n - start1) / (stop1 - start1) * (stop2 - start2) + start2
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    const WIN_WIDTH:  u32 = 600;
    const WIN_HEIGHT: u32 = 600;
    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("forust", [WIN_WIDTH, WIN_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let input = "y = x + y";
    const MIN: i32 = -100;
    const MAX: i32 =  100;
    let dims = (MIN, MAX);
    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        input: input.to_string(),
        range: forust::evaluate(input, dims, dims)
            .iter()
            .map(|[x, y]| [
                map_val(*x, MIN as f64, MAX as f64, -(WIN_WIDTH as f64), WIN_WIDTH as f64), 
                map_val(*y, MIN as f64, MAX as f64, WIN_HEIGHT as f64, -(WIN_HEIGHT as f64))])
            .collect(),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        
        
        if let Some(args) = e.text_args() {
            app.handle_keys(&args);
        }
        
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}