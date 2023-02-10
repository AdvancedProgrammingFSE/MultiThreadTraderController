use std::fs;
use relm4::RelmApp;
use crate::App::{AppInput, AppModel};

mod TraderSelector;
mod App;
mod Consts;
mod TraderState;
mod GlobalMessages;
mod RunningTraderItem;
mod RunningTradersContainer;
use serde::Deserialize;
use crate::Consts::{TraderProcessInfo, VisualizerProcessInfo};

#[derive(Debug, Deserialize)]
struct JsonFileStructure {
    traders : Vec<(String,String)>,
    visualizers : Vec<(String,String)>
}

pub fn parseInputFile() -> AppInput{
    
    let content = fs::read_to_string("./config.json");
    
    if let Ok(c) = content {
        let ss : Result<JsonFileStructure, _> = serde_json::from_str(c.as_str());
        if let Ok(j) = ss {
            return AppInput {
                visualizers: j.visualizers.iter().map(|(k,v)| VisualizerProcessInfo{
                    label: k.to_string(),
                    path: v.to_string(),
                }).collect(),
                traders: j.traders.iter().map(|(k,v)| TraderProcessInfo{
                    label: k.to_string(),
                    path: v.to_string(),
                }).collect(),
            }
        }
        
    }
    
    AppInput{
        visualizers: vec![],
        traders: vec![],
    }
}

fn main() {
    let app = RelmApp::new("testApp");
    app.run::<AppModel>(parseInputFile());
}
