use std::rc::Rc;
use relm4::WorkerController;

pub type GlobalState<T> = Rc<WorkerController<T>>;

pub const GLOBAL_MARGIN : i32 = 10;

#[derive(Debug)]
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