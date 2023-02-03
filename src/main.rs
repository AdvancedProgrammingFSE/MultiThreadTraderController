use relm4::RelmApp;
use crate::App::AppModel;

mod TraderSelector;
mod App;
mod Consts;
mod TraderState;
mod GlobalMessages;

fn main() {
    let app = RelmApp::new("testApp");
    app.run::<AppModel>(());
}
