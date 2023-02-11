use relm4::*;
use crate::misc::TraderProcessInfo;
use crate::global_messages::GlobalMsg;

#[derive(Debug)]
pub struct TraderStateModel {
	current_trader: Option<String>,
	running_traders: Vec<TraderProcessInfo>
}

impl Worker for TraderStateModel {
	type Init = ();
	type Input = GlobalMsg;
	type Output = GlobalMsg;
	
	fn init(_init: Self::Init, _sender: ComponentSender<Self>) -> Self {
		TraderStateModel {
			current_trader: None,
			running_traders: vec![],
		}
	}
	
	fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
		match msg {
			
			GlobalMsg::SetSelectedTrader(s) => {
				self.current_trader = Some(s.clone());
			},
			
			GlobalMsg::GetSelectedTrader(mut f) => {
				println!("get request received");
				f.call(self.current_trader.clone());
			}
			
			GlobalMsg::AddRunningTraders(s) => {
				self.running_traders.push(TraderProcessInfo {
					label: s.label.clone(),
					path: s.path.clone(),
				})
			}
			
			_ => {}
		}
	}
}

