extern crate cursive;
extern crate reqwest;
extern crate serde;

use cursive::{Cursive, CbSink};
use cursive::traits::*;
use cursive::views::{Panel, BoxView, LinearLayout, DummyView, TextView, EditView, Button, ListView, SelectView, Dialog};
use cursive::view::{ViewWrapper, SizeConstraint};
use cursive::align::*;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::Write;
use serde_json::Value;

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
    view: BoxView<Panel<LinearLayout>>,
    client: Arc<mmapi::Client>,

}

impl ViewWrapper for CoinSelectionView {
    cursive::wrap_impl!(self.view: BoxView<Panel<LinearLayout>>);
}

impl CoinSelectionView {
    fn new(client: Arc<mmapi::Client>) -> Self {
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
            client,
        }
    }
}

pub fn get_current_coins_list() -> HashMap<String, Coin> {
    let mut response = reqwest::get("https://raw.githubusercontent.com/jl777/coins/master/coins").expect("Unable to get coins json");
    let list: Vec<Coin> = serde_json::from_str(response.text().unwrap().as_str()).unwrap();

    let mut map = HashMap::new();

    for coin in list {
        map.insert(String::from(&coin.coin), coin);
    };

    map
}

pub fn get_mm2_coins() -> Vec<String> {
    let hm = get_current_coins_list();
    let values = hm.values().collect::<Vec<&Coin>>();

    let strings = values.iter()
        .filter(|coin| coin.mm2.is_some())
        .map(|coin| coin.coin.clone()
        ).collect::<Vec<String>>();

    strings
}

pub fn create_coins_file(vec: Vec<String>) {
    let hm = get_current_coins_list();

    let mut selection = HashMap::new();
    vec.iter().for_each(|ticker| {
        selection.insert(ticker.clone(), hm.get(ticker).unwrap());
    });
    let selection = selection.values().collect::<Vec<_>>();

    let mut file = File::create("coins").unwrap();
    file.write(serde_json::to_string(&selection).unwrap().as_bytes()).expect("could not create coins file");
}

pub fn load_coins_file() -> Vec<String> {
    // if this is the first start and coins file doesn't exist yet:
    if let Err(_) = fs::read_to_string("coins") {
        create_coins_file(
            get_mm2_coins()
        )
    }

    let coins = fs::read_to_string("coins").expect("failed to read coins file");
    let list: Vec<Coin> = serde_json::from_str(&coins).unwrap();

    list.iter()
        .map(|coin| coin.coin.clone() )
        .collect::<Vec<String>>().to_owned()
}

pub fn get_electrum_coins() -> Vec<String> {
    let mut response = reqwest::get("https://api.github.com/repos/jorian/coins/contents/electrums").expect("Unable to get list of electrums");
    let list: Value = response.json().expect("unable to convert electrums to json");
    let coinslist = list.as_array().expect("unable to convert electrum json Value to list");

    coinslist.iter()
        .map(|coin| coin["name"].as_str().unwrap().to_string())
        .collect::<Vec<String>>()
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