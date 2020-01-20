use cursive::Cursive;
use std::sync::mpsc;
use crate::controller::ControllerMessage;
use cursive::event::Key;
use cursive::views::{TextArea, Dialog, TextView, SelectView, Button};
use crate::login_v::LoginView;

pub struct Ui {
    cursive: Cursive,
    ui_rx: mpsc::Receiver<UiMessage>,
    pub ui_tx: mpsc::Sender<UiMessage>,
    controller_tx: mpsc::Sender<ControllerMessage>,
}

pub enum UiMessage {
    Balance(String),
    StartMainLayer,
    ElectrumStarted((String, String, String)),
    OrderbookSelectCoin(String, Vec<String>),
    OrderbookUpdateCoinSelect(String, String, String, String)
}

impl Ui {
    pub fn new(controller_tx: mpsc::Sender<ControllerMessage>) -> Self {
        let (ui_tx, ui_rx) = mpsc::channel::<UiMessage>();

        let mut cursive: Cursive = Cursive::default();
        let controller_tx_clone = controller_tx.clone();

        cursive.add_global_callback('q', move |s| {
            controller_tx_clone.send(ControllerMessage::StopMarketmaker);
            s.quit();
        });

        let mut ui = Ui {
            cursive,
            ui_tx,
            ui_rx,
            controller_tx
        };

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
                UiMessage::Balance(balance) => {
                    let mut output = self.cursive
                        .find_id::<TextView>("output")
                        .unwrap();
                    output.set_content(balance);
                },
                UiMessage::StartMainLayer => {
                    let main_view = crate::main_v::create(self.controller_tx.clone());
                    self.cursive.pop_layer();
                    self.cursive.add_layer(main_view);
                },
                UiMessage::ElectrumStarted((coin, address, balance)) => {
                    // TODO add address in menu
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
                    sv.set_on_submit(move |siv, label: &str| {
                        controller_tx_clone.send(ControllerMessage::SelectSide(side.clone(), label.into()))
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
                }

            }
        }

        // Step the UI
        self.cursive.step();

        true
    }
}