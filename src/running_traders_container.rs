use gtk4::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};
use relm4::factory::FactoryVecDeque;
use crate::misc::{VisualizerProcessInfo};
use crate::global_messages::GlobalMsg;
use crate::running_trader_item::RunningTraderItem;


#[derive(Debug)]
pub struct RunningTradersContainer {
	running_traders: FactoryVecDeque<RunningTraderItem>,
	visualizers : Vec<VisualizerProcessInfo>
}

impl SimpleComponent for RunningTradersContainer {
	type Input = GlobalMsg;
	type Output = GlobalMsg;
	type Init = Vec<VisualizerProcessInfo>;
	type Root = gtk::Box;
	type Widgets = ();
	
	fn init_root() -> Self::Root {
		gtk::Box::new(gtk::Orientation::Vertical,5)
	}
	
	// define the structure of the controller
	fn init(init: Self::Init, root: &Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
		let model = Self {
			running_traders: FactoryVecDeque::new(gtk::Box::new(gtk::Orientation::Vertical, 5), sender.input_sender()),
			visualizers: init
		};
		
		let frame = gtk::Frame::builder()
			.label("Selected Traders")
			.build();
		
		let frame_box = model.running_traders.widget();
		frame.set_child(Some(frame_box));
		let _ = root.append(&frame);
		
		ComponentParts { model, widgets: () }
	}
	
	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		match message {
			GlobalMsg::AddRunningTraders(t) => {
				self.running_traders.guard().push_front((t,self.visualizers.to_vec()));
			},
			_ => {}
		}
	}
}