use cursive::views::{ResizedView, Panel, LinearLayout, DummyView, TextView, EditView, Button};
use cursive::view::{ViewWrapper, SizeConstraint};
use std::sync::mpsc;
use crate::controller::ControllerMessage;
use cursive::align::HAlign;
use cursive::traits::{Identifiable, Boxable};
use crate::coin_selection_v;

pub struct LoginView {
    view: ResizedView<Panel<LinearLayout>>,
}

impl ViewWrapper for LoginView {
    cursive::wrap_impl!(self.view: ResizedView<Panel<LinearLayout>>);
}

impl LoginView {
    pub fn new(controller_tx: mpsc::Sender<ControllerMessage>) -> Self {
        let mut loginview = ResizedView::with_full_width(Panel::new(
            LinearLayout::horizontal()
                .child(ResizedView::with_fixed_width(10, DummyView))
                .child(
                    LinearLayout::vertical()
                        .child(
                            TextView::new("\n\n\nEnter your passphrase:")
                                .h_align(HAlign::Center))
                        .child(DummyView)
                        .child(EditView::new()
                            .secret()
                            .with_name("passphrase")
                            .full_width()
                        )
                        .child(DummyView)
                        .child(ResizedView::with_min_height(10, {
                            LinearLayout::horizontal()
                                .child(ResizedView::with_full_width(DummyView))
                                .child(Button::new("Coins",  move |siv| {
                                    let coinselection = coin_selection_v::CoinSelectionView::new();
                                    siv.add_layer(coinselection);
                                }))
                                .child(DummyView)
                                .child(Button::new("Next", move |siv| {
                                    let pp = siv.find_id::<EditView>("passphrase").unwrap();
                                    let pp = pp.get_content().to_string();
                                    controller_tx.send(ControllerMessage::StartMainLayer(pp));
                                }))
                        }
                        ))
                )
                .child(ResizedView::with_fixed_width(10, DummyView))
        ).title("Login")
        );

        loginview.set_height(SizeConstraint::Fixed(18));

        LoginView {
            view: loginview,
        }
    }
}