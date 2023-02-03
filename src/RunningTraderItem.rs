use gtk4::glib::clone;
use gtk4::prelude::*;
use gtk4::{gio, ListItem, Orientation, StringObject};
use relm4::*;
use relm4::factory::{DynamicIndex, FactoryComponent, FactoryView};
use crate::GlobalMessages::GlobalMsg;

#[derive(Debug)]
pub struct RunningTraderItem {
	trader : String,
	visualizers : Vec<String>,
	selected_visualizer : Option<String>
}

pub struct RunningTraderItemWidgets {
	label : gtk::Label,
	visualizers_dropdown : gtk::DropDown,
	run_visualizer_button : gtk::Button
}

impl FactoryComponent for RunningTraderItem {
	type ParentWidget = gtk::Box;
	type ParentInput = GlobalMsg;
	type CommandOutput = ();
	type Input = GlobalMsg;
	type Output = GlobalMsg;
	type Init = (String,Vec<String>);
	type Root = gtk::Box;
	type Widgets = RunningTraderItemWidgets;
	
	fn init_model(init: Self::Init, index: &DynamicIndex, sender: FactorySender<Self>) -> Self {
		Self{
			trader : init.0,
			visualizers : init.1,
			selected_visualizer : None
		}
	}
	
	fn init_root(&self) -> Self::Root {
		gtk::Box::new(gtk::Orientation::Horizontal, 5)
	}
	
	fn init_widgets(&mut self, index: &DynamicIndex, root: &Self::Root, returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget, sender: FactorySender<Self>) -> Self::Widgets {
		
		let label = gtk::Label::builder()
			.label(self.trader.as_str())
			.build();
		
		// convert a Vec<String> to a &[&str]
		let vec_strs = self.visualizers.iter().map(|x|  x.as_str() ).collect::<Vec<&str>>().clone();
		let strs : &[&str] = vec_strs.as_slice();
		
		// define the list model for the dropdown
		let visualizers_list_model = gtk4::StringList::new(strs);
		
		let visualizers_dropdown = gtk::DropDown::builder()
			.model(&visualizers_list_model)
			.width_request(150)
			.build();
		
		visualizers_dropdown.connect_selected_item_notify(clone!(@strong sender, @strong visualizers_dropdown => move |_| {
            if let Some(obj) = visualizers_dropdown.selected_item() {
                if let Ok(s) = obj.dynamic_cast::<StringObject>() {
                    sender.input(GlobalMsg::SetSelectedVisualizer(s.string().to_string()));
                }
            }
        }));
		
		let run_visualizer_button = gtk4::Button::with_label("Run Visualizer");
		run_visualizer_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(GlobalMsg::RunVisualizerPressed);
		}));
		
		root.append(&label);
		root.append(&visualizers_dropdown);
		root.append(&run_visualizer_button);
		
		RunningTraderItemWidgets {
			label,
			visualizers_dropdown,
			run_visualizer_button,
		}
	}
	
	fn update(&mut self, msg: Self::Input, sender: FactorySender<Self>) {
		match msg {
			GlobalMsg::SetSelectedVisualizer(v) => self.selected_visualizer = Some(v),
			GlobalMsg::RunVisualizerPressed => {
				match &self.selected_visualizer {
					None => {println!("no visualizer selected")}
					Some(v) => {println!("start run {}", v)}
				}
				// code to spawn a visualizer and attach to it the pipe of the trader
			},
			_ => {}
		}
	}
}