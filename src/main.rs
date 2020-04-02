extern crate cursive;
extern crate cursive_table_view;
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
    let controller = Controller::new();
    match controller {
        Ok(mut controller) => controller.run(),
        Err(e) => println!("Error: {}", e),
    }
}