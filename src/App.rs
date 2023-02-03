use gtk4::prelude::*;
use relm4::*;

// List of actions to which the components respond
#[derive(Debug)]
enum AppMsg {

}

// Values and other Components stored inside this Component
struct AppModel {

}

// List of widgets inside the component
struct AppWidgets {

}

impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;
    type Root = gtk::Box;
    
    // initialize the root widget where the rest of the component will reside
    fn init_root() -> <Self as relm4::SimpleComponent>::Root {
        gkt::Box::builder().orientation(gtk4::Orientation::Vertical).build()
    }
    
    // define how the state of the component change or what to do in response to an event
    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
        
        }
    }
    
    // define how the component is structured
    fn init(init: Self::Init, root: &Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = AppModel {
        
        };
        
        // initialize widgets and components
        
        let widgets = AppWidgets {
        
        };
        
        ComponentParts { model, widgets }
    }
 }
     