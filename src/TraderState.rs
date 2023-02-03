use std::borrow::Borrow;
use gtk4::prelude::*;
use relm4::*;
use crate::GlobalMessages::GlobalMsg;

pub struct TraderStateModel {
	currentTrader : Option<String>,
	runningTraders : Vec<String>
}

impl Worker for TraderStateModel {
	type Init = ();
	type Input = GlobalMsg;
	type Output = GlobalMsg;
	
	fn init(init: Self::Init, sender: ComponentSender<Self>) -> Self {
		TraderStateModel {
			currentTrader : None,
			runningTraders: vec![],
		}
	}
	
	fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
		match msg {
			
			GlobalMsg::SetCurrentTrader(s) => {
				self.currentTrader = s.clone();
			},
			
			GlobalMsg::GetCurrentTrader(mut f) => {
				println!("get request received");
				f.call(self.currentTrader.clone());
			}
			
			_ => {}
		}
	}
}

