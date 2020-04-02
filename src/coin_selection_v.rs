use cursive::views::{ResizedView, DummyView, Button, Panel, LinearLayout, SelectView, Dialog};
use cursive::align::HAlign;
use cursive::Cursive;
use cursive::view::ViewWrapper;
use cursive::traits::{Identifiable, Scrollable};
use crate::coin_management::*;


pub struct CoinSelectionView {
    view: ResizedView<Panel<LinearLayout>>,
}

impl ViewWrapper for CoinSelectionView {
    cursive::wrap_impl!(self.view: ResizedView<Panel<LinearLayout>>);
}

impl CoinSelectionView {
    pub fn new() -> Self {
        fn add_coin(siv: &mut Cursive, s: &String) {
            siv.call_on_name("selected_coins", |view: &mut SelectView<String>| {
                view.add_item_str(String::from(s))
            });

            siv.call_on_name("available_coins", |view: &mut SelectView<String>| {
                view.remove_item(view.selected_id().unwrap())
            });
        }

        fn remove_coin(siv: &mut Cursive, s: &String) {
            siv.call_on_name("available_coins", |view: &mut SelectView<String>| {
                view.add_item_str(String::from(s));
                view.sort();
            });

            siv.call_on_name("selected_coins", |view: &mut SelectView<String>| {
                view.remove_item(view.selected_id().unwrap())
            });
        }

        fn close_coin_selection(siv: &mut Cursive) {
            let mut selection: Vec<String> = vec![];

            siv.call_on_name("selected_coins", |view: &mut SelectView<String>| {
                selection = view.iter()
                    .map(|select| select.1.clone() )
                    .collect::<Vec<_>>();
            });

            if selection.len() < 2 {
                siv.add_layer(
                    Dialog::info("You need to select at least 2 coins to continue")
                )
            } else {
                create_coins_file(selection);

                siv.pop_layer();
            }
        }

        // should call marketmaker crate here
        let mut mm2_coins = get_mm2_coins();
        let coins_file_coins = load_coins_file();

        // exclude already active coins from the available coins list
        mm2_coins.retain(|coin| !coins_file_coins.contains(coin));

        let mut available_coins = SelectView::<String>::new()
            .h_align(HAlign::Left)
            .autojump();

        let mut selected_coins = SelectView::<String>::new()
            .h_align(HAlign::Left)
            .autojump();

        available_coins.add_all_str(mm2_coins);
        available_coins.sort();
        selected_coins.add_all_str(coins_file_coins);
        selected_coins.sort();

        CoinSelectionView {
            view: ResizedView::with_full_screen(
                Panel::new(LinearLayout::horizontal()
                    .child(ResizedView::with_min_width(20, DummyView))
                    .child(
                        LinearLayout::vertical()
                            .child(
                                ResizedView::with_full_height(
                                    LinearLayout::horizontal()
                                        .child(
                                            ResizedView::with_min_width(18, Panel::new(
                                                available_coins
                                                    .on_submit(add_coin)
                                                    .with_name("available_coins")
                                                    .scrollable()
                                            ).title("Available"))
                                        )
                                        .child(ResizedView::with_min_width(6, DummyView))
                                        .child(
                                            ResizedView::with_min_width(18, Panel::new(
                                                selected_coins
                                                    .on_submit(remove_coin)
                                                    .with_name("selected_coins")
                                                    .scrollable()
                                            ).title("Selected"))
                                        )
                                )
                            )
                            .child(
                                Button::new("Apply", close_coin_selection)
                            )
                    )
                    .child(ResizedView::with_min_width(20, DummyView))
                ).title("Select coins")
            ),
        }
    }
}