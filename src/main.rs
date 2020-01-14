extern crate cursive;
extern crate reqwest;

use cursive::{Cursive, CbSink};
use cursive::traits::*;
use cursive::views::{Panel, BoxView, LinearLayout, DummyView, TextView, EditView, Button, ListView};
use cursive::view::{ViewWrapper, SizeConstraint};
use cursive::align::HAlign::Center;
use std::sync::Arc;
use std::collections::HashMap;


fn main() {
    let client = Arc::new(mmapi::Client::new("23y4g23g23jgjgjH3GJHGJKHg34"));
    // marketmaker: marketmaker::Marketmaker::new().with_coins.etc.etc

    let mut siv: Cursive = Cursive::default();

    let cb_sink = siv.cb_sink().clone();
    let loginview = LoginView::new(client.clone(), cb_sink);

    siv.add_layer(loginview);
    siv.set_autorefresh(true);

    siv.run();
}

struct LoginView {
    view: BoxView<Panel<LinearLayout>>,
    client: Arc<mmapi::Client>,
}

impl ViewWrapper for LoginView {
    cursive::wrap_impl!(self.view: BoxView<Panel<LinearLayout>>);
}

impl LoginView {
    fn new(client: Arc<mmapi::Client>, cb_sink: CbSink) -> Self {
        let client2 = client.clone();
        let mut loginview = BoxView::with_full_width(Panel::new(
            LinearLayout::horizontal()
            .child(BoxView::with_fixed_width(10, DummyView).squishable())
            .child(
                LinearLayout::vertical()
                    .child(
                        TextView::new("\n\n\nEnter your passphrase:")
                            .h_align(Center))
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
                                //
                                let coinselection = CoinSelectionView::new(client2.clone());
                                siv.add_layer(coinselection);
                            }))
                            .child(DummyView)
                            .child(Button::new("Next", |_| ()))
                    }
                    ).squishable())
            )
            .child(BoxView::with_fixed_width(10, DummyView).squishable())
        ).title("Login")
        );

        loginview.set_height(SizeConstraint::Fixed(18));

        LoginView {
            view: loginview,
            client,
        }
    }
}

struct CoinSelectionView {
    view: BoxView<LinearLayout>,
    client: Arc<mmapi::Client>,

}

impl ViewWrapper for CoinSelectionView {
    cursive::wrap_impl!(self.view: BoxView<LinearLayout>);
}

impl CoinSelectionView {
    fn new(client: Arc<mmapi::Client>) -> Self {
        // should call marketmaker crate here

        let mut response = reqwest::get("https://raw.githubusercontent.com/jl777/coins/master/coins").expect("Unable to get coins json");
        let list: Vec<Coin> = serde_json::from_str(response.text().unwrap().as_str()).unwrap();

        let mut map = HashMap::new();

        for coin in list {
            map.insert(String::from(&coin.coin), coin);
        };

        CoinSelectionView {
            view: BoxView::with_full_screen(
            LinearLayout::horizontal()
                .child(
                    BoxView::with_min_width(10, {
                        let lv = ListView::new();
                    })
                ).child(Button::new("Quit", |siv| {
                    siv.pop_layer();
                }))
            ),
            client,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coin {
    pub coin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpcport: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubtype: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p2shtype: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiftype: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mm2: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txversion: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwintered: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txfee: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etomic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimals: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confpath: Option<String>,
    #[serde(rename = "isPos", skip_serializing_if = "Option::is_none")]
    pub is_pos: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taddr: Option<u16>,
    #[serde(rename = "nSPV", skip_serializing_if = "Option::is_none")]
    pub n_spv: Option<String>,
}