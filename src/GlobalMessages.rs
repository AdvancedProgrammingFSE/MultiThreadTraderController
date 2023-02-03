use std::fmt::{Debug, Formatter};
use gtk4::glib::Object;
use gtk4::prelude::*;
use relm4::*;

#[derive(Debug)]
pub enum GlobalMsg {
	
	SetSelectedTrader(String),
	GetSelectedTrader(CallBack<Option<String>>),
	GetSelectedTraderResponse(Option<String>),
	
	AddRunningTraders(String),
	GetRunningTraders(CallBack<Vec<String>>),
	GetRunningTradersResponse(Vec<String>),
	
	SetSelectedVisualizer(String),
	
	
	RunVisualizerPressed,
	RunTraderPressed
}


// struct used to send a callback function with a Msg
pub struct CallBack<I>{
	pub callback: Box<dyn FnMut(I)>
}

impl<I> CallBack<I> {
	pub fn call(&mut self,s: I) {
		(self.callback)(s);
	}
	pub fn From (f: impl FnMut(I) + 'static) -> Self {
			CallBack {
				callback : Box::new(f)
			}
	}
}

impl<I> Debug for CallBack<I> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Callback function")
	}
}

unsafe impl<I> Send for CallBack<I> {}




