use cursive::views::{DummyView, Button, TextView, BoxView, LinearLayout, ListView};
use cursive::align::HAlign;
use cursive::theme::{BaseColor, Color};
use cursive::utils::markup::StyledString;
use cursive::traits::{View, Identifiable};

use crate::coin_management::*;

pub fn create() -> Box<dyn View> {
    let overview = BoxView::with_full_screen(
        LinearLayout::horizontal()
            .child(
                BoxView::with_min_width(10, {
                    let mut lv = ListView::new();
                    let coin_list = load_coins_file();
                    let electrum_list = get_electrum_coins();

                    lv.clear();

                    for coin in coin_list {
//                        let client = client.clone();

                        let coin_clone = coin.clone();
                        if electrum_list.contains(&coin) {
                            lv.add_child(&coin, LinearLayout::horizontal()
                                .child(BoxView::with_full_width(DummyView))
                                .child(TextView::new(
                                    StyledString::styled("not activated", Color::Dark(BaseColor::Red))
                                )
                                    .h_align(HAlign::Right)
                                    .with_id(format!("electrum_balance_{}", coin_clone.clone())))
                                .child(BoxView::with_fixed_width(3, DummyView))
                                .child({
                                    Button::new("activate", move |siv| {
//                                        let electrum = client.electrum(&coin_clone, true).unwrap();

//                                        siv.call_on_id(&format!("electrum_balance_{}", &coin_clone), |tv: &mut TextView| {
//                                            tv.set_content(&electrum.balance.unwrap())
//                                        });
//
//                                        siv.call_on_id(&format!("electrum_activate_{}", &coin_clone), |b: &mut Button| {
//                                            b.disable();
//                                        });
                                        ()
                                    }).with_id(format!("electrum_activate_{}", String::from(&coin)))
                                })
                                .child(DummyView));
                        } else {
                            lv.add_child(&coin, DummyView)
                        }
                    }

                    lv.with_id("electrum_coins")
                })
            )
    );

    Box::new(overview.with_id("coins_view"))
}