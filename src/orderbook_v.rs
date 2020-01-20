use cursive::align::{HAlign};
use std::cmp::Ordering;
use cursive::views::{DummyView, LinearLayout, BoxView, Button, Dialog, SelectView, TextView, Panel};
use cursive::Cursive;
use std::sync::{mpsc};
use cursive::traits::{View, Identifiable, Boxable};
use crate::controller::ControllerMessage;
use cursive_aligned_view::{Alignable, AlignedView};
use cursive::theme::Effect;
use crate::coin_management::load_coins_file;

pub fn create(controller_tx: mpsc::Sender<ControllerMessage>) -> Box<dyn View> {
    let controller_tx_clone = controller_tx.clone();
    let overview = BoxView::with_full_screen(
        LinearLayout::horizontal()
            .child(
                AlignedView::with_center_left(
                    BoxView::with_full_screen(
                        Panel::new(
                            LinearLayout::vertical().child(
                                LinearLayout::horizontal()
                                    .child(BoxView::with_full_width(DummyView))
                                    .child(BoxView::with_fixed_height(1,
                                        Button::new("Select coin", move |s| {
                                            let mut selected_coins = load_coins_file();

                                            let mut sv = SelectView::<String>::new();
                                            sv.add_all_str(selected_coins);
                                            let controller_tx_clone2 = controller_tx_clone.clone();
                                            sv.set_on_submit(move |siv, label: &str| {
                                                controller_tx_clone2.send(ControllerMessage::SelectAsk(label.into()))
                                            });

                                            s.add_layer(Dialog::around(sv).title("Select"));
                                        })
                                    ))
                                    .child(BoxView::with_full_width(DummyView))
                            ).child(LinearLayout::horizontal().child(
                                TextView::new("TEST")
                                    .h_align(HAlign::Center)
                                )
                            )
                        )
                    )
                )
            )
            .child(LinearLayout::vertical()
                .child(AlignedView::with_bottom_center(BoxView::with_full_screen(Panel::new(DummyView))))
                .child(AlignedView::with_top_center(BoxView::with_full_screen(Panel::new(DummyView)))))
            .child(AlignedView::with_center_right(BoxView::with_full_screen(Panel::new(DummyView))))
    );
//        LinearLayout::vertical()
//            .child(
//                LinearLayout::horizontal()
//                    .child(
//                        TextView::new("Select pair (base / rel):"))
//                    .child(DummyView)
//                    .child(Button::new("select", |siv| {
//                        let mut _rel = String::new();
//
//                        fn set_button_label(siv: &mut Cursive, label: &String) {
//                            println!("{}", label);
//                            siv.call_on_id("base-btn", |btn: &mut Button| { btn.set_label_raw(label) });
//
//                            siv.pop_layer();
//                        }
//
//                        let mut selected_coins = crate::coin_management::load_coins_file();
//
//                        selected_coins.retain(|coin| {
//                            siv.call_on_id("rel-btn", |btn: &mut Button| {
//                                _rel = String::from(btn.label());
//                            });
//
//                            coin.ne(&_rel)
//                        });
//
//                        let mut sv = SelectView::<String>::new();
//                        sv.add_all_str(selected_coins);
//
//                        sv.set_on_submit(set_button_label);
//
//                        siv.add_layer(Dialog::around(sv))
//                    }).with_id("base-btn"))
//                    .child(DummyView)
//                    .child(TextView::new("/"))
//                    .child(DummyView)
//                    .child(Button::new("select", |siv| {
//                        let mut _base = String::new();
//
//                        fn set_button_label(siv: &mut Cursive, label: &String) {
//                            println!("{}", label);
//                            siv.call_on_id("rel-btn", |btn: &mut Button| { btn.set_label_raw(label) });
//
//                            siv.pop_layer();
//                        }
//
//                        let mut selected_coins = crate::coin_management::load_coins_file();
//
//                        selected_coins.retain(|coin| {
//                            siv.call_on_id("base-btn", |btn: &mut Button| {
//                                _base = String::from(btn.label());
//                            });
//
//                            coin.ne(&_base)
//                        });
//
//                        let mut sv = SelectView::<String>::new();
//                        sv.add_all_str(selected_coins);
//
//                        sv.set_on_submit(set_button_label);
//
//                        siv.add_layer(Dialog::around(sv))
//                    }).with_id("rel-btn")))
//            .child(DummyView)
//            .child(
//                AlignedView::with_bottom_center(
//                    BoxView::with_fixed_height(28, BoxView::with_full_width(
//                        LinearLayout::horizontal()
//                            .child(AlignedView::with_center_right(BoxView::with_full_screen(Panel::new(DummyView))))
//                            .child(AlignedView::with_center_left(BoxView::with_full_screen(Panel::new(DummyView))))
//                    ))
//                )
//            )
//    );
//            .child(
//                LinearLayout::horizontal()
//                    .child(
//                        Button::new("Fetch orderbook", move |siv| {
//                            let mut base = String::new();
//                            let mut rel = String::new();
//
////                         fetch orderbook for pair
//                            let cb_sink = siv.cb_sink().clone();
//
//                            siv.call_on_id("base-btn", |btn: &mut Button| {
//                                base = String::from(btn.label());
//                            });
//
//                            siv.call_on_id("rel-btn", |btn: &mut Button| {
//                                rel = String::from(btn.label());
//                            });
//
////                        let client = rpc::client::Client::new();
//                            let orderbook = client.orderbook(&base, &rel).unwrap();
//
//                            siv.call_on_id("ask-side", | tbl: &mut TableView<Ask, BasicColumn> | {
//                                tbl.clear();
//
//                                for ask in orderbook.asks.unwrap() {
//                                    let ask: Ask = ask.into();
//                                    tbl.insert_item(ask.to_owned())
//                                }
//                            });
//
//                            siv.call_on_id("bid-side", | tbl: &mut TableView<Bid, BasicColumn> | {
//                                tbl.clear();
//
////                            for bid in orderbook.bids.unwrap() {
//////                                tbl.insert_item(bid)
////                            }
//                            });
//
//                        }).with_id("fetch_orderbook_btn"))
//                    .child(BoxView::with_full_width(DummyView)))
//            .child(DummyView)
//            .child(LinearLayout::horizontal()
//                .child(TableView::<Ask, BasicColumn>::new()
//                    .column(BasicColumn::Maxvolume, "Volume", |c| c.align(HAlign::Center))
//                    .column(BasicColumn::Price, "Price", |c| {
//                        c.ordering(Ordering::Less).width_percent(25)
//                    })
//                    .default_column(BasicColumn::Price)
//                    .with_id("ask-side")
//                    .full_screen())
//                .child(DummyView)
//                .child(TableView::<Bid, BasicColumn>::new()
//                    .column(BasicColumn::Price, "Price", |c| {
//                        c.ordering(Ordering::Greater).width_percent(25)
//                    })
//                    .column(BasicColumn::Maxvolume, "Volume", |c| c.align(HAlign::Center))
//                    .with_id("bid-side")
//                    .full_screen())
//            ));

    Box::new(overview.with_id("orderbook"))
}