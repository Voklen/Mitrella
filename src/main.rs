use egui::Ui;

use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            plot(ui);
        });
    }
}

fn plot(ui: &mut Ui) {
    use egui_plot::{Line, PlotPoints};
    let n = 128;
    let sin = Line::new(
        (0..=n)
            .map(|i| {
                use std::f64::consts::TAU;
                let x = egui::remap(i as f64, 0.0..=n as f64, -TAU..=TAU);
                [x, x.sin()]
            })
            .collect::<PlotPoints>(),
    );
    let cos = Line::new(
        (0..=n)
            .map(|i| {
                use std::f64::consts::TAU;
                let x = egui::remap(i as f64, 0.0..=n as f64, -TAU..=TAU);
                [x, x.cos()]
            })
            .collect::<PlotPoints>(),
    );
    egui_plot::Plot::new("example_plot")
        .show_axes(true)
        .data_aspect(1.0)
        .show(ui, |plot_ui| {
            plot_ui.line(sin);
            plot_ui.line(cos)
        })
        .response;
}
