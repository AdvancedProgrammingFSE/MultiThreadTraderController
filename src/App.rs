use std::rc::Rc;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::subclass::prelude::InstanceStructExt;
use relm4::*;
use crate::Consts::GLOBAL_MARGIN;
use crate::TraderSelector::TraderSelectorModel;
use crate::TraderState::TraderStateModel;

// List of actions to which the components respond
#[derive(Debug)]
pub enum AppMsg {}

// Values and other Components stored inside this Component
pub struct AppModel {
	tradersDropDown: Controller<TraderSelectorModel>
}

// List of widgets inside the component
pub struct AppWidgets {}

impl SimpleComponent for AppModel {
	type Init = ();
	type Input = AppMsg;
	type Output = ();
	type Widgets = AppWidgets;
	type Root = gtk::Window;
	
	// initialize the root widget where the rest of the component will reside
	fn init_root() -> <Self as relm4::SimpleComponent>::Root {
		gtk::Window::builder()
            .title("app")
            .build()
	}
	
	// define how the state of the component change or what to do in response to an event
	fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
		match msg {}
	}
	
	// define how the component is structured
	fn init(init: Self::Init, root: &Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
		
		let stt= Rc::new(TraderStateModel::builder().detach_worker(()).detach());
		
		let model = AppModel {
			tradersDropDown: TraderSelectorModel::builder().launch((vec!["trader1".to_string(), "trader2".to_string()], stt.clone())).detach()
        };
		
		// initialize widgets and components
		let rootbox = gtk::Box::builder()
		    .orientation(gtk::Orientation::Vertical)
            .spacing(10)
            .margin_start(GLOBAL_MARGIN)
            .margin_bottom(GLOBAL_MARGIN)
            .margin_end(GLOBAL_MARGIN)
            .margin_top(GLOBAL_MARGIN)
            .build();
        root.set_child(Some(&rootbox));
		
		
        
		rootbox.append(model.tradersDropDown.widget());
        
		let widgets = AppWidgets {};
		
		ComponentParts { model, widgets }
	}
}
     