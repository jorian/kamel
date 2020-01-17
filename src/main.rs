extern crate cursive;
extern crate reqwest;
extern crate serde;
extern crate dirs;

//////// COMPONENTS
mod controller;
mod ui;

//////// VIEWS:
mod active_coins_v;
mod coin_selection_v;
mod coin_management;
mod login_v;
mod main_v;
mod menu_v;
mod orderbook_v;

use crate::controller::Controller;

fn main() {
    // userpass -> random generator
//    let client = Arc::new(mmapi::Client::new("23y4g23g23jgjgjH3GJHGJKHg34"));
    // marketmaker: marketmaker::Marketmaker::new().with_coins.etc.etc

    let controller = Controller::new();
    match controller {
        Ok(mut controller) => controller.run(),
        Err(e) => println!("Error: {}", e),
    }
}