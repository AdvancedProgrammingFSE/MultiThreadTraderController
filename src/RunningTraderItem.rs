use std::borrow::Borrow;
use std::process::Stdio;
use gtk4::glib::clone;
use gtk4::prelude::*;
use gtk4::{gio, ListItem, Orientation, StringObject};
use gtk4::AccessibleRole::Command;
use relm4::*;
use relm4::factory::{DynamicIndex, FactoryComponent, FactoryView};
use crate::Consts::{GLOBAL_MARGIN, TraderProcessInfo, VisualizerProcessInfo};
use crate::GlobalMessages::GlobalMsg;



#[derive(Debug)]
pub struct RunningTraderItem {
	trader : TraderProcessInfo,
	visualizers : Vec<VisualizerProcessInfo>,
	selected_visualizer : Option<VisualizerProcessInfo>
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
	type Init = (TraderProcessInfo,Vec<VisualizerProcessInfo>);
	type Root = gtk::Box;
	type Widgets = RunningTraderItemWidgets;
	
	// initialize the model of the compoonent (itself basically)
	fn init_model(init      : Self::Init,
	              index     : &DynamicIndex,
	              sender    : FactorySender<Self>) -> Self
	{
		Self{
			trader : init.0,
			visualizers : init.1,
			selected_visualizer : None
		}
	}
	
	// crete the root widget, where all the other widgets will be added
	fn init_root(&self) -> Self::Root {
		gtk::Box::builder()
			.orientation(gtk::Orientation::Horizontal)
			.spacing(5)
			.margin_end(GLOBAL_MARGIN)
			.margin_bottom(GLOBAL_MARGIN)
			.margin_top(GLOBAL_MARGIN)
			.margin_start(GLOBAL_MARGIN)
			.build()
	}
	
	// initialize and add all the other widgets
	// also connect events
	fn init_widgets(&mut self,
	                index           : &DynamicIndex,
	                root            : &Self::Root,
	                returned_widget : &<Self::ParentWidget as FactoryView>::ReturnedWidget,
	                sender          : FactorySender<Self>) -> Self::Widgets
	{
		
		let label = gtk::Label::builder()
			.label(&*self.trader.get_label())
			.build();
		
		// convert a Vec<String> to a &[&str] because the dropdwawn widget need it
		let vec_strs = self.visualizers
			.iter()
			.map(|x|  x.get_label().as_str() )
			.collect::<Vec<&str>>()
			.clone();
		
		let strs : &[&str] = vec_strs.as_slice();
		
		// define the list model for the dropdown
		let visualizers_list_model = gtk4::StringList::new(strs);
		
		let visualizers_dropdown = gtk::DropDown::builder()
			.selected(0)
			.model(&visualizers_list_model)
			.width_request(150)
			.build();
		
		// respond to select event
		visualizers_dropdown.connect_selected_item_notify(
			clone!(@strong sender, @strong visualizers_dropdown =>
				move |_| {
		            if let Some(obj) = visualizers_dropdown.selected_item() {
		                if let Ok(s) = obj.dynamic_cast::<StringObject>() {
							let vis = self.visualizers.iter().find(|v| v.label == s.string().to_string());
							if let Some(v) = vis {
								sender.input(GlobalMsg::SetSelectedVisualizer(VisualizerProcessInfo{
									label : v.get_label(),
									path : v.get_path()
								}));
							}
		                }
		            }
		        }
			)
		);
		
		// add a button to start the visualizer
		let run_visualizer_button = gtk4::Button::with_label("Run");
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
					Some(visualizer) => {
						
						let trader_process = std::process::Command::new(self.trader.get_path())
							.stdout(Stdio::piped()).spawn();
						
						if let Ok(process) = trader_process {
							let visualizer_process = std::process::Command::new(visualizer.get_path())
								.stdin(process.stdout.unwrap());
						}
					}
				}
			},
			_ => {}
		}
	}
}