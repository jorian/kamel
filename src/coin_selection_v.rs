use cursive::views::{BoxView, DummyView, Button, Panel, LinearLayout, SelectView, Dialog};
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::collections::HashMap;
use cursive::align::HAlign;
use cursive::Cursive;
use cursive::view::ViewWrapper;
use std::io::Write;
use cursive::traits::{Identifiable, Scrollable};
use serde::{Serialize, Deserialize};
use crate::coin_management::*;


pub struct CoinSelectionView {
    view: BoxView<Panel<LinearLayout>>,
}

impl ViewWrapper for CoinSelectionView {
    cursive::wrap_impl!(self.view: BoxView<Panel<LinearLayout>>);
}

impl CoinSelectionView {
    pub fn new() -> Self {
        fn add_coin(siv: &mut Cursive, s: &String) {
            siv.call_on_id("selected_coins", |view: &mut SelectView<String>| {
                view.add_item_str(String::from(s))
            });

            siv.call_on_id("available_coins", |view: &mut SelectView<String>| {
                view.remove_item(view.selected_id().unwrap())
            });
        }

        fn remove_coin(siv: &mut Cursive, s: &String) {
            siv.call_on_id("available_coins", |view: &mut SelectView<String>| {
                view.add_item_str(String::from(s));
                view.sort();
            });

            siv.call_on_id("selected_coins", |view: &mut SelectView<String>| {
                view.remove_item(view.selected_id().unwrap())
            });
        }

        fn close_coin_selection(mut siv: &mut Cursive) {
            let mut selection: Vec<String> = vec![];

            siv.call_on_id("selected_coins", |view: &mut SelectView<String>| {
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
            view: BoxView::with_full_screen(
                Panel::new(LinearLayout::horizontal()
                    .child(BoxView::with_min_width(20, DummyView).squishable())
                    .child(
                        LinearLayout::vertical()
                            .child(
                                BoxView::with_full_height(
                                    LinearLayout::horizontal()
                                        .child(
                                            BoxView::with_min_width(18, Panel::new(
                                                available_coins
                                                    .on_submit(add_coin)
                                                    .with_id("available_coins")
                                                    .scrollable()
                                            ).title("Available"))
                                        )
                                        .child(BoxView::with_min_width(6, DummyView).squishable())
                                        .child(
                                            BoxView::with_min_width(18, Panel::new(
                                                selected_coins
                                                    .on_submit(remove_coin)
                                                    .with_id("selected_coins")
                                                    .scrollable()
                                            ).title("Selected"))
                                        )
                                )
                            )
                            .child(
                                Button::new("Apply", close_coin_selection)
                            )
                    )
                    .child(BoxView::with_min_width(20, DummyView).squishable())
                ).title("Select coins")
            ),
        }
    }
}