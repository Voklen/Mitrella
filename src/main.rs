mod plotting;

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
struct MitrellaApp {
    equations: Vec<String>,
}

impl MitrellaApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        Self::default()
    }
}

impl eframe::App for MitrellaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("equations_panel").show(ctx, |ui| {
            if ui.button("Add equation").clicked() {
                self.equations.push(String::new())
            }
            for equation in &mut self.equations {
                ui.text_edit_singleline(equation);
            }
        });
        egui::CentralPanel::default().show(&ctx, |ui| {
            let equations = vec![|x: f64| x.sin(), |x: f64| x.cos()];
            plotting::plot(ui, equations);
        });
    }
}
