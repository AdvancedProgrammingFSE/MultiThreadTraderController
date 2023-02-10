use std::borrow::Borrow;
use gtk4::prelude::*;
use relm4::*;
use crate::Consts::TraderProcessInfo;
use crate::GlobalMessages::GlobalMsg;

#[derive(Debug)]
pub struct TraderStateModel {
	currentTrader : Option<String>,
	runningTraders : Vec<TraderProcessInfo>
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
			
			GlobalMsg::SetSelectedTrader(s) => {
				self.currentTrader = Some(s.clone());
			},
			
			GlobalMsg::GetSelectedTrader(mut f) => {
				println!("get request received");
				f.call(self.currentTrader.clone());
			}
			
			GlobalMsg::AddRunningTraders(s) => {
				self.runningTraders.push(TraderProcessInfo {
					label: s.label.clone(),
					path: s.path.clone(),
				})
			}
			
			_ => {}
		}
	}
}

