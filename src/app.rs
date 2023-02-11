use std::rc::Rc;

use gtk4::prelude::*;

use relm4::*;
use crate::misc::{GLOBAL_MARGIN, TraderProcessInfo, VisualizerProcessInfo};
use crate::trader_selector::{TraderSelectorInput, TraderSelectorModel};
use crate::trader_state::TraderStateModel;

// List of actions to which the components respond
#[derive(Debug)]
pub enum AppMsg {}

// Values and other Components stored inside this Component
pub struct AppModel {
	traders_drop_down: Controller<TraderSelectorModel>
}

// List of widgets inside the component
pub struct AppWidgets {}

pub struct AppInput {
	pub(crate) visualizers : Vec<VisualizerProcessInfo>,
	pub(crate) traders : Vec<TraderProcessInfo>
}

impl SimpleComponent for AppModel {
	type Input = ();
	type Output = ();
	type Init = AppInput;
	type Root = gtk::Window;
	type Widgets = AppWidgets;
	
	// initialize the root widget where the rest of the component will reside
	fn init_root() -> <Self as relm4::SimpleComponent>::Root {
		gtk::Window::builder()
            .title("app")
            .build()
	}
	
	// define how the component is structured
	fn init(init     : Self::Init,
	        root     : &Self::Root,
	        _sender  : ComponentSender<Self>)
		-> ComponentParts<Self>
	{
		// create and share the worker that represent the global state
		let stt= Rc::new(
			TraderStateModel::builder()
			.detach_worker(())
			.detach()
		);
		
		let model = AppModel {
			traders_drop_down: TraderSelectorModel::builder()
				.launch(TraderSelectorInput{
					visualizers: init.visualizers,
					traders: init.traders,
					state: stt.clone(),
				})
				.detach()
        };
		
		// initialize widgets and components
		let root_box = gtk::Box::builder()
		    .orientation(gtk::Orientation::Vertical)
            .spacing(10)
            .margin_start(GLOBAL_MARGIN)
            .margin_bottom(GLOBAL_MARGIN)
            .margin_end(GLOBAL_MARGIN)
            .margin_top(GLOBAL_MARGIN)
            .build();
        root.set_child(Some(&root_box));
		
		
		let _ = root_box.append(model.traders_drop_down.widget());
  
		let widgets = AppWidgets {};
		
		ComponentParts { model, widgets }
	}
}
     