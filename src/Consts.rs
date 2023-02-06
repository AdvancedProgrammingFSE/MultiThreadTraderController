use std::rc::Rc;
use relm4::WorkerController;

pub type GlobalState<T> = Rc<WorkerController<T>>;

pub const GLOBAL_MARGIN : i32 = 10;