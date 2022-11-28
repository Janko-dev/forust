#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, plot::Line};
use egui::widgets::plot::{Plot};
use rusty_graph::algebra_eval;

fn main() {

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(GraphApp::default())),
    );
}

struct GraphApp {
    range: Vec<[f64; 2]>,
    input: String
}

impl Default for GraphApp {
    fn default() -> Self {
        let input = "x = y".to_string();
        let range = algebra_eval::evaluate(&input, [-100, 100], [-100, 100]);
        
        Self { 
            input,
            range
        }
    }
}

impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui app");
            ui.horizontal(|ui| {
                ui.label("Expression: ");
                ui.text_edit_singleline(&mut self.input);
            });
            if ui.button("Evaluate").clicked() {
                // do stuff
                self.range = algebra_eval::evaluate(&self.input, [-100, 100], [-100, 100]);
            }

            ui.spacing();

            let line = Line::new(self.range.clone());
            Plot::new("new plot")
                .view_aspect(2.0)
                .allow_boxed_zoom(false)
                .allow_scroll(false)
                .show(ui, |plot_ui| plot_ui.line(line));
        });
        
    }
}