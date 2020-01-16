use cursive::traits::{View, Identifiable, Boxable};
use cursive::views::{LinearLayout, ViewBox, StackView, Panel};

pub fn create() -> Box<dyn View> {
    let menu = crate::menu_v::create();

    let active_coins = crate::active_coins_v::create();

    let stack = StackView::new()
        .layer(active_coins)
        .with_id("root_stack")
        .full_height();

    let main = LinearLayout::horizontal()
        .child(Panel::new(ViewBox::new(menu)))
        .child(Panel::new(stack));

    Box::new(main)
}