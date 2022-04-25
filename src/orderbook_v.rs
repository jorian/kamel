use crate::controller::ControllerMessage;
use cursive::align::HAlign;
use cursive::traits::{Boxable, Identifiable, View};
use cursive::views::{Button, DummyView, LinearLayout, Panel, ResizedView, TextView};
use cursive_aligned_view::AlignedView;
use cursive_table_view::{TableView, TableViewItem};
use std::cmp::Ordering;
use std::sync::mpsc;

pub fn create(controller_tx: mpsc::Sender<ControllerMessage>) -> Box<dyn View> {
    let controller_tx_clone = controller_tx.clone();
    let controller_tx_clone2 = controller_tx.clone();

    let overview = ResizedView::with_full_screen(
        LinearLayout::horizontal()
            .child(AlignedView::with_center_left(
                ResizedView::with_full_screen(Panel::new(
                    LinearLayout::vertical()
                        .child(
                            LinearLayout::horizontal()
                                .child(ResizedView::with_full_width(DummyView))
                                .child(ResizedView::with_fixed_height(
                                    1,
                                    Button::new("Select coin", move |_s| {
                                        controller_tx_clone
                                            .clone()
                                            .send(ControllerMessage::FetchEnabledCoins(
                                                "ask".to_string(),
                                            ))
                                            .unwrap();
                                    })
                                    .with_name("orderbook_ask_select_btn"),
                                ))
                                .child(ResizedView::with_full_width(DummyView)),
                        )
                        .child(
                            LinearLayout::horizontal()
                                .child(TextView::new("").with_name("orderbook_ask_address")),
                        )
                        .child(DummyView.fixed_height(5))
                        .child(
                            {
                                let mut view =
                                    TableView::<mmapi::response::Ask, BasicColumn>::new();
                                view.disable();
                                view
                            }
                            .column(BasicColumn::Maxvolume, "Volume", |c| c.align(HAlign::Right))
                            .column(BasicColumn::Price, "Price", |c| {
                                c.ordering(Ordering::Less).width_percent(40)
                            })
                            .default_column(BasicColumn::Price)
                            .with_name("ask-side")
                            .full_screen(),
                        ),
                )),
            ))
            .child(
                LinearLayout::vertical().child(AlignedView::with_bottom_center(
                    ResizedView::with_full_screen(Panel::new(
                        LinearLayout::vertical().child(TextView::new("Hello")),
                    )),
                )),
            )
            .child(AlignedView::with_center_left(
                ResizedView::with_full_screen(Panel::new(
                    LinearLayout::vertical()
                        .child(
                            LinearLayout::horizontal()
                                .child(ResizedView::with_full_width(DummyView))
                                .child(ResizedView::with_fixed_height(
                                    1,
                                    Button::new("Select coin", move |_s| {
                                        controller_tx_clone2
                                            .clone()
                                            .send(ControllerMessage::FetchEnabledCoins(
                                                "bid".to_string(),
                                            ))
                                            .unwrap();
                                    })
                                    .with_name("orderbook_bid_select_btn"),
                                ))
                                .child(ResizedView::with_full_width(DummyView)),
                        )
                        .child(
                            LinearLayout::horizontal()
                                .child(TextView::new("").with_name("orderbook_bid_address")),
                        )
                        .child(DummyView.fixed_height(5))
                        .child(
                            {
                                let mut view =
                                    TableView::<mmapi::response::Bid, BasicColumn>::new();
                                view.disable();
                                view
                            }
                            .column(BasicColumn::Maxvolume, "Volume", |c| c.align(HAlign::Right))
                            .column(BasicColumn::Price, "Price", |c| {
                                c.ordering(Ordering::Less).width_percent(40)
                            })
                            .default_column(BasicColumn::Price)
                            .with_name("bid-side")
                            .full_screen(),
                        ),
                )),
            )),
    );

    Box::new(overview.with_name("orderbook"))
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum BasicColumn {
    Price,
    Maxvolume,
}

impl TableViewItem<BasicColumn> for mmapi::response::Ask {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::Price => format!("{:.8}", &self.price.parse::<f64>().unwrap()),
            BasicColumn::Maxvolume => format!("{:.8}", &self.maxvolume.parse::<f64>().unwrap()),
        }

        // String::from("TEST")
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            BasicColumn::Price => self
                .price
                .parse::<f64>()
                .unwrap()
                .partial_cmp(&other.price.parse().unwrap())
                .unwrap(),
            BasicColumn::Maxvolume => self.maxvolume.partial_cmp(&other.maxvolume).unwrap(),
        }
    }
}

impl TableViewItem<BasicColumn> for mmapi::types::response::Bid {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::Price => format!("{:.8}", &self.price.parse::<f64>().unwrap()),
            BasicColumn::Maxvolume => format!("{:.8}", &self.maxvolume.parse::<f64>().unwrap()),
        }

        // String::from("TEST")
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            BasicColumn::Price => other
                .price
                .parse::<f64>()
                .unwrap()
                .partial_cmp(&self.price.parse().unwrap())
                .unwrap(),
            BasicColumn::Maxvolume => self.maxvolume.partial_cmp(&other.maxvolume).unwrap(),
        }
    }
}
