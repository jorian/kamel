use cursive::views::{BoxView, Panel, LinearLayout, DummyView, TextView, EditView, Button};
use cursive::view::{ViewWrapper, SizeConstraint};
use std::sync::mpsc;
use crate::controller::ControllerMessage;
use cursive::align::HAlign;
use cursive::traits::{Identifiable, Boxable};
use crate::coin_selection_v;

pub struct LoginView {
    view: BoxView<Panel<LinearLayout>>,
}

impl ViewWrapper for LoginView {
    cursive::wrap_impl!(self.view: BoxView<Panel<LinearLayout>>);
}

impl LoginView {
    pub fn new(controller_tx: mpsc::Sender<ControllerMessage>) -> Self {
        let mut loginview = BoxView::with_full_width(Panel::new(
            LinearLayout::horizontal()
                .child(BoxView::with_fixed_width(10, DummyView).squishable())
                .child(
                    LinearLayout::vertical()
                        .child(
                            TextView::new("\n\n\nEnter your passphrase:")
                                .h_align(HAlign::Center))
                        .child(DummyView)
                        .child(EditView::new()
                            .secret()
                            .with_id("passphrase")
                            .full_width()
                        )
                        .child(DummyView)
                        .child(BoxView::with_min_height(10, {
                            LinearLayout::horizontal()
                                .child(BoxView::with_full_width(DummyView))
                                .child(Button::new("Coins",  move |siv| {
                                    // do coin fetching in closure, call cb_sink with data?
                                    // https://stackoverflow.com/questions/33662098/cannot-move-out-of-captured-outer-variable-in-an-fn-closure

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
                        ).squishable())
                )
                .child(BoxView::with_fixed_width(10, DummyView).squishable())
        ).title("Login")
        );

        loginview.set_height(SizeConstraint::Fixed(18));

        LoginView {
            view: loginview,
        }
    }
}