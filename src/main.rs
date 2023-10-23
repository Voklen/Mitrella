use egui::Ui;
use egui_plot::{Line, PlotPoints, PlotUi};
use std::f64::consts::TAU;

use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let res = eframe::run_native(
        "Mitrella",
        native_options,
        Box::new(|cc| Box::new(MitrellaApp::new(cc))),
    );
    if let Err(e) = res {
        eprintln!("{e}");
    }
}

#[derive(Default)]
struct MitrellaApp {}

impl MitrellaApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        Self::default()
    }
}

impl eframe::App for MitrellaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            let equations = vec![|x: f64| x.sin(), |x: f64| x.cos()];
            plot(ui, equations);
        });
    }
}

fn plot(ui: &mut Ui, equations: Vec<impl Fn(f64) -> f64>) {
    let lines = equations.into_iter().map(to_line);
    let build_fn = |ui: &mut PlotUi| build_plot(ui, lines);
    egui_plot::Plot::new("main_plot")
        .show_axes(true)
        .data_aspect(1.0)
        .show(ui, build_fn)
        .response;
}

fn to_line(equation: impl Fn(f64) -> f64) -> Line {
    let total_points = 128;
    let plot_point = |n| {
        let x = egui::remap(n as f64, 0.0..=total_points as f64, -TAU..=TAU);
        [x, equation(x)]
    };
    let points: PlotPoints = (0..=total_points).map(plot_point).collect();
    Line::new(points)
}

fn build_plot(plot_ui: &mut PlotUi, lines: impl Iterator<Item = Line>) {
    for line in lines {
        plot_ui.line(line);
    }
}
