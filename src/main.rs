extern crate cursive;
extern crate reqwest;
extern crate serde;
extern crate dirs;

mod login_v;
mod coin_selection_v;
mod controller;
mod ui;

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