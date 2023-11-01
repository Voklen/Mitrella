mod equations;
mod parser;
mod plotting;

use eframe::egui::*;
use equations::Equations;

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
	equations: Equations,
}

impl MitrellaApp {
	fn new(_cc: &eframe::CreationContext<'_>) -> Self {
		// Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
		// Restore app state using cc.storage (requires the "persistence" feature).
		Self::default()
	}
}

impl eframe::App for MitrellaApp {
	fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
		let equations = &mut self.equations;
		SidePanel::left("equations_panel")
			.default_width(200.0)
			.show(ctx, |ui| {
				ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
					ScrollArea::vertical().show(ui, |ui| {
						equations_panel(ui, equations);
					});
				});
			});
		CentralPanel::default().show(&ctx, |ui| {
			plotting::plot(ui, &equations.functions);
		});
	}
}

fn equations_panel(ui: &mut Ui, equations: &mut Equations) {
	if ui.button("Add equation").clicked() {
		equations.push(String::new())
	}
	for index in 0..equations.len() {
		let text_edit = &ui.text_edit_singleline(&mut equations.strings[index]);
		if text_edit.changed() {
			equations.update_func(index);
		};
	}
}
