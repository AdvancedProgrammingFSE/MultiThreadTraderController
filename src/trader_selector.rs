use std::convert::identity;
use gtk4::{StringObject};
use gtk4::glib::{clone};
use relm4::*;
use gtk4::prelude::*;

use crate::misc::{gtk_horizontal_box, TraderProcessInfo, VisualizerProcessInfo};
use crate::global_messages::{GlobalMsg};
use crate::running_traders_container::RunningTradersContainer;

// Values and other Components stored inside this Component
pub struct TraderSelectorModel {
    traders : Vec<TraderProcessInfo>,
    running_traders_list: Controller<RunningTradersContainer>,
    selected_trader: Option<TraderProcessInfo>
}

pub struct TraderSelectorInput {
    pub(crate) visualizers : Vec<VisualizerProcessInfo>,
    pub(crate) traders : Vec<TraderProcessInfo>
}

impl SimpleComponent for TraderSelectorModel {
    type Input = GlobalMsg;
    type Output = GlobalMsg;
    type Init = TraderSelectorInput;
    type Root = gtk::Box;
    type Widgets = ();
    
    // initialize the root widget where the rest of the component will reside
    fn init_root() -> <Self as relm4::SimpleComponent>::Root {
        gtk::Box::builder().orientation(gtk4::Orientation::Horizontal).build()
    }
    
    // define how the component is structured
    fn init(init    : Self::Init,
            root    : &Self::Root,
            sender  : ComponentSender<Self>)
        -> ComponentParts<Self>
    {
        let model = TraderSelectorModel {
            traders : init.traders.clone(),
           // trader_state: init.state.clone(),
            running_traders_list: RunningTradersContainer::builder()
                .launch(init.visualizers)
                .forward(sender.input_sender(),identity),
            selected_trader: init.traders.first().cloned(),
        };
        
        // initialize widgets and components
        let frame = gtk::Frame::builder()
            .label("Trader")
            .sensitive(true)
            .build();
        
        // create a box to give to the frame as a child
        let frame_child_box = gtk_horizontal_box();
        
        // convert a Vec<String> to a &[&str]
        let vec_strs = model.traders
            .iter()
            .map(|x|  x.label.as_str() )
            .collect::<Vec<&str>>()
            .clone();
        
        let strs : &[&str] = vec_strs.as_slice();
        
        // define the list model for the dropdown
        let trader_list_model = gtk4::StringList::new(strs);
        
        let trader_dropdown = gtk::DropDown::builder()
            .selected(0)
            .model(&trader_list_model)
            .width_request(150)
            .build();
        
        let traders_list_clone = model.traders.clone();
        
        // connect the selected item notification to the dropdown
        // this way it can store in the global state the string of the selected trader
        trader_dropdown.connect_selected_item_notify(
            clone!(@strong sender, @strong trader_dropdown =>
                move |_| {
                    if let Some(obj) = trader_dropdown.selected_item() {
                        if let Ok(s) = obj.dynamic_cast::<StringObject>() {
                            
                            // search for the trader with the label equal to s
							let tr = traders_list_clone
								.iter()
								.find(|v| v.label == s.string().to_string());
                            
                            if let Some(t) = tr {
                                sender.input(GlobalMsg::SetSelectedTrader(TraderProcessInfo{
                                    label : t.get_label(),
									path : t.get_path()
                                }));
                            }
                        }
                    }
                }
            )
        );
        
        // define a button to run a trader
        let run_button = gtk4::Button::builder()
            .label("Select")
            .width_request(50)
            .margin_start(100)
            .build();
        run_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(GlobalMsg::RunTraderPressed);
        }));
    
        let _ = frame_child_box.append(&trader_dropdown);
        let _ = frame_child_box.append(&run_button);
        
        frame.set_child(Some(&frame_child_box));
        
        let outer_box = gtk::Box::new(gtk::Orientation::Vertical,5);
        let _ = outer_box.append(&frame);
        let _ = outer_box.append(model.running_traders_list.widget());
    
        let _ = root.append(&outer_box);
        
        ComponentParts { model, widgets: () }
    }
    
    // define how the state of the component change or what to do in response to an event
    fn update(&mut self,
              msg       : Self::Input,
              _sender    : ComponentSender<Self>)
    {
        match msg {
            GlobalMsg::SetSelectedTrader(tr) => self.selected_trader = Some(tr),
            GlobalMsg::RunTraderPressed => {
                match &self.selected_trader {
                    None => { println!("No trader selected") }
                    Some(trader) => {
                        // notify the trader list of the creation of a new trader process
                        self.running_traders_list.emit(GlobalMsg::AddRunningTraders(TraderProcessInfo {
                            label: trader.get_label(),
                            path: trader.get_path(),
                        }));
                    }
                }
            },
            _ => {}
        }
    }
}
     