use relm4::{gtk};

pub const GLOBAL_MARGIN : i32 = 10;

#[derive(Debug,Clone)]
pub struct TraderProcessInfo {
	pub label : String,
	pub path : String
}

impl TraderProcessInfo {
	pub fn get_label(&self) -> String { self.label.clone() }
	pub fn get_path(&self) -> String { self.path.clone() }
}

#[derive(Debug,Clone)]
pub struct VisualizerProcessInfo {
	pub label : String,
	pub path : String
}


impl VisualizerProcessInfo {
	pub fn get_label(&self) -> String { self.label.clone() }
	pub fn get_path(&self) -> String { self.path.clone() }
}

pub fn gtk_horizontal_box() -> gtk::Box {
	gtk::Box::builder()
		.orientation(gtk::Orientation::Horizontal)
		.spacing(5)
		.margin_end(GLOBAL_MARGIN)
		.margin_bottom(GLOBAL_MARGIN)
		.margin_top(GLOBAL_MARGIN)
		.margin_start(GLOBAL_MARGIN)
		.build()
}