use relm4::*;
use gtk4::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
use relm4::factory::FactoryVecDeque;
use crate::Consts::{GlobalState, VisualizerProcessInfo};
use crate::GlobalMessages::GlobalMsg;
use crate::RunningTraderItem::RunningTraderItem;
use crate::TraderState::TraderStateModel;

#[derive(Debug)]
pub struct RunningTradersContainer {
	running_traders: FactoryVecDeque<RunningTraderItem>,
	visualizers : Vec<VisualizerProcessInfo>,
	traders_state: GlobalState<TraderStateModel>
}

impl SimpleComponent for RunningTradersContainer {
	type Input = GlobalMsg;
	type Output = GlobalMsg;
	type Init = (GlobalState<TraderStateModel>,Vec<VisualizerProcessInfo>);
	type Root = gtk::Box;
	type Widgets = ();
	
	fn init_root() -> Self::Root {
		gtk::Box::new(gtk::Orientation::Vertical,5)
	}
	
	// define the structure of the controller
	fn init(init: Self::Init, root: &Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
		let model = Self {
			running_traders: FactoryVecDeque::new(gtk::Box::new(gtk::Orientation::Vertical, 5), sender.input_sender()),
			traders_state: init.0,
			visualizers: init.1
		};
		
		let frame = gtk::Frame::builder()
			.label("Selected Traders")
			.build();
		
		let frame_box = model.running_traders.widget();
		frame.set_child(Some(frame_box));
		root.append(&frame);
		
		ComponentParts { model, widgets: () }
	}
	
	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		match message {
			GlobalMsg::AddRunningTraders(t) => {
				self.running_traders.guard().push_front((t,self.visualizers.to_vec()));
			},
			_ => {}
		}
	}
}