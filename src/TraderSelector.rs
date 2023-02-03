use std::borrow::Borrow;
use std::convert::identity;
use std::ops::Deref;
use std::process::{Command, Stdio};
use std::rc::Rc;
use gtk4::{gio, ListItem, Orientation, StringObject};
use gtk4::glib::{clone, GString};
use relm4::*;
use gtk4::prelude::*;
use gtk4::ShortcutScope::Global;
use crate::Consts::GlobalState;
use crate::GlobalMessages::{CallBack, GlobalMsg};
use crate::RunningTradersContainer::RunningTradersContainer;
use crate::TraderState::{TraderStateModel};

// Values and other Components stored inside this Component
pub struct TraderSelectorModel {
    traders : Vec<String>,
    currentTraderCache : Option<String>,
    traderState : GlobalState<TraderStateModel>,
    runningTradersList : Controller<RunningTradersContainer>
}

// List of widgets inside the component
//pub struct TraderSelectorWidgets {}

impl SimpleComponent for TraderSelectorModel {
    
    type Init = (Vec<String>,GlobalState<TraderStateModel>);
    type Input = GlobalMsg;
    type Output = GlobalMsg;
    type Widgets = ();
    type Root = gtk::Box;
    
    // initialize the root widget where the rest of the component will reside
    fn init_root() -> <Self as relm4::SimpleComponent>::Root {
        gtk::Box::builder().orientation(gtk4::Orientation::Horizontal).build()
    }
    
    
    // define how the state of the component change or what to do in response to an event
    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            GlobalMsg::SetSelectedTrader(tr) => {self.traderState.emit(GlobalMsg::SetSelectedTrader(tr.clone()))}
            
            GlobalMsg::RunTraderPressed => {self.traderState.emit(GlobalMsg::GetSelectedTrader(
                CallBack::From(clone!(@strong sender => move |x:Option<String>| {
                    sender.input(GlobalMsg::GetSelectedTraderResponse(x))
                }))
            ))}
            
            GlobalMsg::GetSelectedTraderResponse(tr) => {
                match tr {
                    None => {println!("None")}
                    Some(t) => {
                        // Calls to spawn the trader
                        // todo
                        
                        self.traderState.emit(GlobalMsg::AddRunningTraders(t.clone()));
                        self.runningTradersList.emit(GlobalMsg::AddRunningTraders(t.clone()));
                        //sender.input(GlobalMsg::AddRunningTraders(t.clone()));
                    }
                }
            },
            _ => {}
        }
    }
    
    // define how the component is structured
    fn init(init: Self::Init, root: &Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = TraderSelectorModel {
            traders : init.0,
            traderState : init.1.clone(),
            currentTraderCache: None,
            runningTradersList : RunningTradersContainer::builder().launch(init.1.clone()).forward(sender.input_sender(),identity)
        };
        
        // initialize widgets and components
        let frame = gtk::Frame::builder()
            .label("Trader")
            .sensitive(true)
            .build();
        
        // create a box to give to the frame as a child
        let frame_child_box = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        
        // convert a Vec<String> to a &[&str]
        let vec_strs = model.traders.iter().map(|x|  x.as_str() ).collect::<Vec<&str>>().clone();
        let strs : &[&str] = vec_strs.as_slice();
        
        // define the list model for the dropdown
        let trader_list_model = gtk4::StringList::new(strs);
        
        let trader_dropdown = gtk::DropDown::builder()
            .model(&trader_list_model)
            .width_request(150)
            .build();
        
        // connect the selected item notification to the dropdown
        // this way it can store in the global state the string of the selected trader
        trader_dropdown.connect_selected_item_notify(clone!(@strong sender, @strong trader_dropdown => move |_| {
            if let Some(obj) = trader_dropdown.selected_item() {
                if let Ok(s) = obj.dynamic_cast::<StringObject>() {
                    sender.input(GlobalMsg::SetSelectedTrader(s.string().to_string()));
                }
            }
        }));
        
        
        // define a button to run a trader
        let run_button = gtk4::Button::builder()
            .label("Run")
            .width_request(50)
            .margin_start(100)
            .build();
        run_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(GlobalMsg::RunTraderPressed);
        }));
        
        frame_child_box.append(&trader_dropdown);
        frame_child_box.append(&run_button);
        
        frame.set_child(Some(&frame_child_box));
        
        let outer_box = gtk::Box::new(gtk::Orientation::Vertical,5);
        outer_box.append(&frame);
        outer_box.append(model.runningTradersList.widget());
        
        root.append(&outer_box);
        
        //let widgets = TraderSelectorWidgets {};
        
        ComponentParts { model, widgets: () }
    }
}
     