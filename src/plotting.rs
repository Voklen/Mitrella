use egui::Ui;
use egui_plot::{Line, PlotPoints, PlotUi};
use std::f64::consts::TAU;

pub fn plot(ui: &mut Ui, equations: &[impl Fn(f64) -> Option<f64>]) {
	let clean_iter = equations.into_iter();
	let lines = clean_iter.map(to_line);
	let build_fn = |ui: &mut PlotUi| build_plot(ui, lines);
	egui_plot::Plot::new("main_plot")
		.show_axes(true)
		.data_aspect(1.0)
		.show(ui, build_fn)
		.response;
}

fn to_line(equation: impl Fn(f64) -> Option<f64>) -> Line {
	let total_points = 128;
	let plot_point = |n| {
		let x = egui::remap(n as f64, 0.0..=total_points as f64, -TAU..=TAU);
		match equation(x) {
			Some(res) => Some([x, res]),
			None => None,
		}
	};
	let points: PlotPoints = (0..=total_points).map(plot_point).flatten().collect();
	Line::new(points)
}

fn build_plot(plot_ui: &mut PlotUi, lines: impl Iterator<Item = Line>) {
	for line in lines {
		plot_ui.line(line);
	}
}
