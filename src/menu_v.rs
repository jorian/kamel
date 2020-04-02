use super::cursive::Cursive;
use super::cursive::view::View;
use super::cursive::views::{ResizedView, SelectView, StackView, LinearLayout, TextView};
use super::cursive::align::HAlign;
//use super::cursive::traits::Identifiable;
use super::cursive::view::Identifiable;

pub fn create() -> Box<dyn View> {
    let mut main_menu = SelectView::new()
        .h_align(HAlign::Left)
        .with_name("main_menu");

    main_menu.get_mut().add_item("Coins", "active_coins");
    main_menu.get_mut().add_item("Orderbook", "orderbook");

    let change_view = |s: &mut Cursive, v: &&str| {
        if *v == "" {
            return;
        }

        let _ = s.call_on_name("root_stack", |sv: &mut StackView| {
            let pos = sv.find_layer_from_name(v).unwrap();
            sv.move_to_front(pos);
        });
    };

    main_menu.get_mut().set_on_select(change_view);

    let main_menu = LinearLayout::vertical()
        .child(ResizedView::with_full_height(main_menu))
        .child(TextView::new("++++++++++++++"))
        .child(TextView::new("Q to quit"));

    Box::new(main_menu)
}