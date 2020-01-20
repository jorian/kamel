use cursive::align::{HAlign};
use std::cmp::Ordering;
use cursive::views::{DummyView, LinearLayout, ResizedView, Button, Dialog, SelectView, TextView, Panel};
use cursive::{Cursive, With};
use std::sync::{mpsc};
use cursive::traits::{View, Identifiable, Boxable};
use crate::controller::ControllerMessage;
use cursive_aligned_view::{Alignable, AlignedView};
use cursive::theme::Effect;
use crate::coin_management::load_coins_file;
use cursive_table_view::{TableView, TableColumn, TableViewItem};

pub fn create(controller_tx: mpsc::Sender<ControllerMessage>) -> Box<dyn View> {
//    let controller_tx_clone = controller_tx.clone();
//    let controller_tx_clone2 = controller_tx.clone();

    fn create_orderbook_side(side: String, controller_tx: mpsc::Sender<ControllerMessage>) -> AlignedView<ResizedView<Panel<LinearLayout>>> {
        let controller_tx_clone = controller_tx.clone();

        let side_clone = side.clone();

        AlignedView::with_center_left(
            ResizedView::with_full_screen(
                Panel::new(
                    LinearLayout::vertical()
                        .child(
                        LinearLayout::horizontal()
                            .child(ResizedView::with_full_width(DummyView))
                            .child(ResizedView::with_fixed_height(1, Button::new("Select coin", move |s| {
                                controller_tx_clone.send(ControllerMessage::FetchEnabledCoins(side_clone.clone()));
                            }).with_name(&format!("orderbook_{}_select_btn", &side))))
                            .child(ResizedView::with_full_width(DummyView))
                        )
                        .child(LinearLayout::horizontal().child(
                        TextView::new("")
                            .with_name(format!("orderbook_{}_address", &side))
                        ))
                        .child(DummyView.fixed_height(5))
                        .child(TableView::<mmapi::response::Ask, BasicColumn>::new()
                            .column(BasicColumn::Maxvolume, "Volume", |c| c.align(HAlign::Center))
                            .column(BasicColumn::Price, "Price", |c| {
                        c.ordering(Ordering::Less).width_percent(25)
                    })
                    .default_column(BasicColumn::Price)
                    .with_name(format!("{}-side", &side))
                    .full_screen()))

            )
        )
    }

    let overview = ResizedView::with_full_screen(
        LinearLayout::horizontal()
            .child(
                create_orderbook_side("ask".to_string(), controller_tx.clone())
            )
            .child(LinearLayout::vertical()
                .child(AlignedView::with_bottom_center(ResizedView::with_full_screen(Panel::new(DummyView))))
//                .child(AlignedView::with_top_center(ResizedView::with_full_screen(Panel::new(DummyView)))))
            )
            .child(
                create_orderbook_side("bid".to_string(), controller_tx.clone())
            )
    );

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
//                        }).with_name("fetch_orderbook_btn"))
//                    .child(ResizedView::with_full_width(DummyView)))
//            .child(DummyView)
//            .child(LinearLayout::horizontal()
//                .child(TableView::<Ask, BasicColumn>::new()
//                    .column(BasicColumn::Maxvolume, "Volume", |c| c.align(HAlign::Center))
//                    .column(BasicColumn::Price, "Price", |c| {
//                        c.ordering(Ordering::Less).width_percent(25)
//                    })
//                    .default_column(BasicColumn::Price)
//                    .with_name("ask-side")
//                    .full_screen())
//                .child(DummyView)
//                .child(TableView::<Bid, BasicColumn>::new()
//                    .column(BasicColumn::Price, "Price", |c| {
//                        c.ordering(Ordering::Greater).width_percent(25)
//                    })
//                    .column(BasicColumn::Maxvolume, "Volume", |c| c.align(HAlign::Center))
//                    .with_name("bid-side")
//                    .full_screen())
//            ));

    Box::new(overview.with_name("orderbook"))
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum BasicColumn {
    Price,
    Maxvolume
}

impl TableViewItem<BasicColumn> for mmapi::response::Ask {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::Price => self.price.to_string(),
            BasicColumn::Maxvolume => self.maxvolume.to_string()
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering where
        Self: Sized {
        match column {
            BasicColumn::Price => self.price.partial_cmp(&other.price).unwrap(),
            BasicColumn::Maxvolume => self.maxvolume.partial_cmp(&other.maxvolume).unwrap()
        }
    }
}
