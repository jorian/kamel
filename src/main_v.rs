use cursive::traits::{View, Identifiable, Boxable};
use cursive::views::{LinearLayout, ViewBox, StackView, Panel};
use crate::controller::ControllerMessage;
use std::sync::mpsc;

pub fn create(controller_tx: mpsc::Sender<ControllerMessage>) -> Box<dyn View> {
    let menu = crate::menu_v::create();

    let active_coins = crate::active_coins_v::create(controller_tx.clone());
    let orderbook = crate::orderbook_v::create(controller_tx.clone());

    let stack = StackView::new()
        .layer(orderbook)
        .layer(active_coins)
        .with_id("root_stack")
        .full_height();

    let main = LinearLayout::horizontal()
        .child(Panel::new(ViewBox::new(menu)))
        .child(Panel::new(stack));

    Box::new(main)
}