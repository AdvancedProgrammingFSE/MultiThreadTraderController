use relm4::RelmApp;
use crate::App::AppModel;

mod TraderSelector;
mod App;
mod Consts;
mod TraderState;
mod GlobalMessages;
mod RunningTraderItem;
mod RunningTradersContainer;

fn main() {
    let app = RelmApp::new("testApp");
    app.run::<AppModel>(());
}
