use cursive::views::{DummyView, Button, TextView, BoxView, LinearLayout, ListView};
use cursive::align::HAlign;
use cursive::theme::{BaseColor, Color};
use cursive::utils::markup::StyledString;
use cursive::traits::{View, Identifiable, Boxable};

use std::sync::mpsc;

use crate::coin_management::*;
use crate::controller::ControllerMessage;

pub fn create(controller_tx: mpsc::Sender<ControllerMessage>) -> Box<dyn View> {
    let overview = BoxView::with_full_screen(
        LinearLayout::horizontal()
            .child(
                BoxView::with_min_width(10, {
                    let mut lv = ListView::new();
                    let coin_list = load_coins_file();
                    let electrum_list = get_electrum_coins();

                    lv.clear();

                    for coin in coin_list {
                        let coin_clone = coin.clone();
                        if electrum_list.contains(&coin) {
                            lv.add_child(&coin, LinearLayout::horizontal()
                                .child(BoxView::with_full_width(DummyView))
                                .child(TextView::new("").with_id(format!("electrum_coin_{}", coin_clone.clone())))
                                .child(TextView::new(
                                    StyledString::styled("not activated", Color::Dark(BaseColor::Red)))
                                    .h_align(HAlign::Right)
                                    .with_id(format!("electrum_balance_{}", coin_clone.clone()))
                                    .fixed_width(14))
                                .child(BoxView::with_fixed_width(3, DummyView))
                                .child({
                                    let controller_clone = controller_tx.clone();
                                    Button::new("activate", move |siv| {
                                        controller_clone.send(ControllerMessage::ElectrumActivate(coin_clone.clone()));
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

    Box::new(overview.with_id("active_coins"))
}