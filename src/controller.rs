use std::sync::mpsc;
use crate::ui::{UiMessage, Ui};
use std::{thread, fs};
use std::process::Command;
use std::time::Duration;
use std::fs::File;
use serde::{Serialize, Deserialize};
use cursive::views::Button;

pub struct Controller {
    rx: mpsc::Receiver<ControllerMessage>,
    ui: Ui,
    client: mmapi::Client,
    electrum_enabled: Vec<String>,
}

pub enum ControllerMessage {
    // FetchBalance(String),
    StartMainLayer(String),
    ElectrumActivate(String),
    StopMarketmaker,
    SelectSide(String, String),
    FetchEnabledCoins(String),
    UpdateOrderbook
}

impl Controller {
    pub fn new() -> Result<Controller, String> {
        let (tx, rx) = mpsc::channel::<ControllerMessage>();

        Ok(Controller {
            rx,
            ui: Ui::new(tx.clone()),
            // i would need to start marketmaker before starting the controller. which is not possible.
            // i can initialize a client without userpass, and set it later
            client: mmapi::Client::new("23y4g23g23jgjgjH3GJHGJKHg34"),
            electrum_enabled: vec![]
        })
    }

    pub fn run(&mut self) {
        while self.ui.step() {
            // on each step, clear the message queue that the controller receives
            while let Some(message) = self.rx.try_iter().next() {
                match message {
                    // ControllerMessage::FetchBalance(coin) => {
                    //     let balance = self.client.balance(&coin).unwrap();
                    //     self.ui
                    //         .ui_tx
                    //         .send(UiMessage::Balance(balance.balance.unwrap()))
                    //         .unwrap();
                    // },
                    ControllerMessage::StartMainLayer(passphrase) => {
                        let userhome = dirs::home_dir().expect("Unable to get userhome");
                        let userpass = self.client.get_userpass().clone();


                        let mm2_json = Mm2Json::create(
                            &userpass,
                            passphrase.as_str(),
                            userhome.to_str().unwrap()
                        );

                        mm2_json.create_mm2_json();

                        thread::spawn( move || {
                            let _mm2client =
                                Command::new("./marketmaker")
                                    .spawn()
                                    .expect("Failed to start marketmaker binary. Does it exist?");

                            thread::sleep(Duration::from_secs(1));
                            std::fs::remove_file("MM2.json").expect("couldn't remove MM2.json after startup");
                        });

                        self.ui
                            .ui_tx
                            .send(UiMessage::StartMainLayer)
                            .unwrap();
                    },
                    ControllerMessage::ElectrumActivate(coin) => {
                        let electrum = self.client.electrum(&coin, true).unwrap();

                        if let Some(error) = electrum.error {
                            // tell the UI to show the error
                            println!("{}", error);
                        } else {
                            self.electrum_enabled.push(coin.clone());
                            self.ui
                                .ui_tx
                                .send(UiMessage::ElectrumStarted((electrum.coin.unwrap(), electrum.address.unwrap(), electrum.balance.unwrap())))
                                .unwrap();
                        }
                    },
                    ControllerMessage::StopMarketmaker => {
                        self.client.stop().unwrap();
                    },
                    ControllerMessage::SelectSide(side, coin) => {
                        let balance = self.client.balance(&coin).unwrap();
                        if let Some(error) = balance.error {
                            // the error could be that the coin was not enabled.
                            // tell the UI to show the error
                            println!("{}", error);
                        } else {
                            if !self.electrum_enabled.contains(&coin) {
                                let _electrum = self.client.electrum(&coin, true).unwrap();
                            } else {
                                let address = balance.address.unwrap();
                                let balance = balance.balance.unwrap();

                                self.ui.ui_tx.send(UiMessage::OrderbookUpdateCoinSelect(side, balance, address, coin))
                                    .unwrap();
                            }
                        }
                    },
                    ControllerMessage::FetchEnabledCoins(side) => {
                        self.ui.ui_tx.send(UiMessage::OrderbookSelectCoin(side.into(), self.electrum_enabled.clone()))
                            .unwrap();
                    },
                    ControllerMessage::UpdateOrderbook => {
                        println!("Update me");
                        let mut base = String::new();
                        let mut rel = String::new();
                        self.ui.cursive.call_on_name("orderbook_ask_select_btn", |btn: &mut Button| {
                            base = String::from(btn.label());
                            dbg!(&base);
                        });
                        self.ui.cursive.call_on_name("orderbook_bid_select_btn", |btn: &mut Button| {
                            rel = String::from(btn.label());
                            dbg!(&rel);

                        });

                        if !base.is_empty() && !rel.is_empty() && !rel.contains('<') && !base.contains('<') {
                            let orderbook = self.client.orderbook(&base, &rel).unwrap();
                            let mut asks = orderbook.asks.unwrap().clone();
                            asks.sort_by(|a, b| a.price.parse::<f64>().unwrap().partial_cmp(&b.price.parse().unwrap()).unwrap());
                            let mut bids = orderbook.bids.unwrap().clone();
                            bids.sort_by(|a, b| b.price.parse::<f64>().unwrap().partial_cmp(&a.price.parse().unwrap()).unwrap());
                            self.ui.ui_tx.send(UiMessage::UpdateOrderbook(asks, bids))
                                .unwrap();
                        }
                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Mm2Json {
    gui: String,
    netid: u16,
    rpc_password: String,
    passphrase: String,
    userhome: String
}

impl Mm2Json {
    pub fn create(rpc_password: &str, passphrase: &str, userhome: &str) -> Self {
        Mm2Json {
            gui: String::from("MM2GUI"),
            netid: 9999,
            rpc_password: rpc_password.to_string(),
            passphrase: passphrase.to_string(),
            userhome: userhome.to_string()
        }
    }

    pub fn create_mm2_json(&self) {
        let _file = File::create("MM2.json").unwrap();
        let serialized_json = serde_json::to_string(&self).unwrap();
        let _ = fs::write("MM2.json", serialized_json);
    }
}