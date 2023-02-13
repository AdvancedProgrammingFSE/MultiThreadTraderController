use std::process::Stdio;
use gtk4::glib::clone;
use gtk4::prelude::*;
use gtk4::{StringObject};
use relm4::*;
use relm4::factory::{DynamicIndex, FactoryComponent, FactoryView};
use crate::misc::{gtk_horizontal_box, TraderProcessInfo, VisualizerProcessInfo};
use crate::global_messages::GlobalMsg;



#[derive(Debug)]
pub struct RunningTraderItem {
	trader : TraderProcessInfo,
	visualizers : Vec<VisualizerProcessInfo>,
	selected_visualizer : Option<VisualizerProcessInfo>
}

pub struct RunningTraderItemWidgets {}

impl FactoryComponent for RunningTraderItem {
	type ParentWidget = gtk::Box;
	type ParentInput = GlobalMsg;
	type CommandOutput = ();
	type Input = GlobalMsg;
	type Output = GlobalMsg;
	type Init = (TraderProcessInfo,Vec<VisualizerProcessInfo>);
	type Root = gtk::Box;
	type Widgets = RunningTraderItemWidgets;
	
	// initialize the model of the component (itself basically)
	fn init_model(init      : Self::Init,
	              _index     : &DynamicIndex,
	              _sender    : FactorySender<Self>) -> Self
	{
		Self{
			trader : init.0,
			visualizers : init.1.clone(),
			selected_visualizer : init.1.first().cloned()
		}
	}
	
	// crete the root widget, where all the other widgets will be added
	fn init_root(&self) -> Self::Root {
		gtk_horizontal_box()
	}
	
	// initialize and add all the other widgets
	// also connect events
	fn init_widgets(&mut self,
	                _index           : &DynamicIndex,
	                root             : &Self::Root,
	                _returned_widget : &<Self::ParentWidget as FactoryView>::ReturnedWidget,
	                sender           : FactorySender<Self>) -> Self::Widgets
	{
		
		let label = gtk::Label::builder()
			.label(&*self.trader.get_label())
			.build();
		
		// convert a Vec<String> to a &[&str] because the dropdown widget need it
		let vec_str = self.visualizers
		                  .iter()
		                  .map(|x|  x.label.as_str() )
		                  .collect::<Vec<&str>>()
		                  .clone();
		
		let array_str: &[&str] = vec_str.as_slice();
		
		// define the list model for the dropdown
		let visualizers_list_model = gtk4::StringList::new(array_str);
		
		let visualizers_dropdown = gtk::DropDown::builder()
			.selected(0)
			.model(&visualizers_list_model)
			.width_request(150)
			.build();
		
		let visualizers_list_clone = self.visualizers.clone();
		
		// respond to select event
		visualizers_dropdown.connect_selected_item_notify(
			clone!(@strong sender, @strong visualizers_dropdown, @strong visualizers_list_clone =>
				move |_| {
		            if let Some(obj) = visualizers_dropdown.selected_item() {
		                if let Ok(s) = obj.dynamic_cast::<StringObject>() {
							
							// search for the visualizer with the label equal to s
							let vis = visualizers_list_clone
								.iter()
								.find(|v| v.label == s.string().to_string());
							
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
		
		let _ = root.append(&label);
		let _ = root.append(&visualizers_dropdown);
		let _ = root.append(&run_visualizer_button);
		
		RunningTraderItemWidgets {
		}
	}
	
	fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
		match msg {
			GlobalMsg::SetSelectedVisualizer(v) => self.selected_visualizer = Some(v),
			GlobalMsg::RunVisualizerPressed => {
				match &self.selected_visualizer {
					None => {println!("no visualizer selected")}
					Some(visualizer) => {
						
						let trader_process = std::process::Command::new(self.trader.get_path())
							.stdout(Stdio::piped()).spawn();
						
						if let Ok(process) = trader_process {
							if let Some(stdout_pipe) = process.stdout {
								let visualizer_process = std::process::Command::new(visualizer.get_path())
									.stdin(stdout_pipe).spawn();
								
								if let Err(err) = visualizer_process {
									println!("visualizer : {} -> {:?}",visualizer.get_path(),err);
								}
							}
						} else {
							println!("trader : {} -> {:?}",self.trader.get_path(),trader_process.err().unwrap());
						}
					}
				}
			},
			_ => {}
		}
	}
}