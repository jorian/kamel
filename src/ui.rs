use cursive::Cursive;
use std::sync::mpsc;
use crate::controller::ControllerMessage;
use cursive::event::Event;
use cursive::views::{Dialog, TextView, SelectView, Button};
use crate::login_v::LoginView;
use std::thread;
use std::time::Duration;
use mmapi::types::response::{Ask, Bid};
use cursive_table_view::TableView;
use crate::orderbook_v::BasicColumn;
use std::process::Command;

pub struct Ui {
    pub cursive: Cursive,
    ui_rx: mpsc::Receiver<UiMessage>,
    pub ui_tx: mpsc::Sender<UiMessage>,
    controller_tx: mpsc::Sender<ControllerMessage>,
}

pub enum UiMessage {
    // Balance(String),
    StartMainLayer,
    ElectrumStarted((String, String, String)),
    OrderbookSelectCoin(String, Vec<String>),
    OrderbookUpdateCoinSelect(String, String, String, String),
    UpdateOrderbook(Vec<Ask>, Vec<Bid>)
}

impl Ui {
    pub fn new(controller_tx: mpsc::Sender<ControllerMessage>) -> Self {
        let (ui_tx, ui_rx) = mpsc::channel::<UiMessage>();

        let mut cursive: Cursive = Cursive::default();
        let controller_tx_clone = controller_tx.clone();
        let controller_tx_clone2 = controller_tx.clone();

        cursive.add_global_callback('q', move |s| {
            controller_tx_clone.send(ControllerMessage::StopMarketmaker).unwrap();
            println!("Stopped q");
            s.quit();
        });

        // cursive.on_event(Event::CtrlChar('c'))

        cursive.add_global_callback(Event::Exit, move |_| {
            println!("Stopped ctrl-c");
            Command::new("pkill")
                .arg("-15")
                .arg("marketmaker")
                .spawn()
                .expect("failed to quit mm2");
            println!("marketmaker killed");
        });

        let mut ui = Ui {
            cursive,
            ui_tx,
            ui_rx,
            controller_tx
        };

        thread::spawn( move || {
            loop {
                controller_tx_clone2.send(ControllerMessage::UpdateOrderbook).unwrap();
                thread::sleep(Duration::from_secs(5));
            }
        });

        // Create all views here, send a controller_tx along with it.
        // whenever a view needs updating, send a message to controller, which sends back
        // the requested information
        let controller_tx_clone = ui.controller_tx.clone();
        ui.cursive.add_layer(LoginView::new(controller_tx_clone.clone()));

        ui.cursive.set_autorefresh(true);

        ui
    }

    /// Step the UI by calling into Cursive's step function, then
    /// processing any UI messages.
    pub fn step(&mut self) -> bool {
        if !self.cursive.is_running() {
            return false;
        }

        // Process any pending UI messages
        // if ui_rx has received any message,
        while let Some(message) = self.ui_rx.try_iter().next() {
            // check which message
            match message {
                // UiMessage::Balance(balance) => {
                //     let mut output = self.cursive
                //         .find_name::<TextView>("output")
                //         .unwrap();
                //     output.set_content(balance);
                // },
                UiMessage::StartMainLayer => {
                    let main_view = crate::main_v::create(self.controller_tx.clone());
                    self.cursive.pop_layer();
                    self.cursive.add_layer(main_view);
                },
                UiMessage::ElectrumStarted((coin, address, balance)) => {
                    self.cursive.call_on_name(&format!("electrum_balance_{}", &coin), |textview: &mut TextView| {
                        textview.set_content(&balance)
                    });

                    self.cursive.call_on_name(&format!("electrum_coin_{}", &coin), |textview: &mut TextView| {
                        textview.set_content(&address)
                    });
                },
                UiMessage::OrderbookSelectCoin(side, coins) => {
                    let mut coins = coins.to_owned();
                    let mut label = String::new();

                    match side.as_str() {
                        "ask" => {
                            coins.retain(|coin| {
                                self.cursive.call_on_name("orderbook_bid_select_btn", |btn: &mut Button| {
                                    label = String::from(btn.label());
                                });

                                coin.ne(&label)
                            });
                        },
                        "bid" => {
                            coins.retain(|coin| {
                                self.cursive.call_on_name("orderbook_ask_select_btn", |btn: &mut Button| {
                                    label = String::from(btn.label());
                                });

                                coin.ne(&label)
                            });
                        },
                        _ => unreachable!()

                    }

                    let mut sv = SelectView::<String>::new();

                    sv.add_all_str(coins);
                    let controller_tx_clone = self.controller_tx.clone();
                    sv.set_on_submit(move |_s, label: &str| {
                        controller_tx_clone.send(ControllerMessage::SelectSide(side.clone(), label.into())).unwrap();
                    });

                    self.cursive.add_layer(Dialog::around(sv)
                        .title("Select")
                        .button("Cancel", |siv| { siv.pop_layer(); }));
                },
                UiMessage::OrderbookUpdateCoinSelect(side, balance, address, coin) => {
                    self.cursive.call_on_name(&format!("orderbook_{}_address", &side), |tv: &mut TextView| {
                        tv.set_content(format!("\n{}: {}", &address, &balance));
                    });

                    self.cursive.call_on_name(&format!("orderbook_{}_select_btn", &side),|btn: &mut Button| {
                        btn.set_label_raw(&coin);
                    });

                    self.cursive.pop_layer();

                    let bid = self.cursive.call_on_name("orderbook_bid_select_btn", |btn: &mut Button| {
                        btn.label().to_string()
                    }).unwrap();

                    let ask = self.cursive.call_on_name("orderbook_ask_select_btn", |btn: &mut Button| {
                        btn.label().to_string()
                    }).unwrap();

                    println!("bid.{}",bid);
                    println!("ask.{}",ask);

                    if !bid.starts_with('<') && !ask.starts_with('<') {
                        self.controller_tx.send(ControllerMessage::UpdateOrderbook).unwrap();
                    }
                },
                UiMessage::UpdateOrderbook(asks, bids) => {
                    self.cursive.call_on_name("ask-side", | tbl: &mut TableView<Ask, BasicColumn> | {
                        tbl.clear();

                        for ask in asks {
                            tbl.insert_item(ask.to_owned())
                        }
                    });

                    self.cursive.call_on_name("bid-side", | tbl: &mut TableView<Bid, BasicColumn> | {
                        tbl.clear();

                        for bid in bids {
                            tbl.insert_item(bid.to_owned())
                        }
                    });
                }
            }
        }

        // Step the UI
        self.cursive.step();

        true
    }
}