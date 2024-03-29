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
    UpdateOutput(String),
    Balance(String),
    StartMainLayer,
    ElectrumStarted((String, String, String)),
    OrderbookSelectCoin(Vec<String>),
    OrderbookUpdateAskCoinSelect(String, String, String)
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

        // Configure a callback
//        ui.cursive.add_global_callback(Key::Esc, move |c| {
//            // When the user presses Escape, send an
//            // UpdatedInputAvailable message to the controller.
//            let input = c.find_id::<TextArea>("input").unwrap();
//            let text = input.get_content().to_owned();
//            controller_tx_clone.send(
//                ControllerMessage::FetchBalance(text))
//                .unwrap();
//        });
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
                UiMessage::UpdateOutput(text) => {
                    // do what is needed upon that message
                    let mut output = self.cursive
                        .find_id::<TextView>("output")
                        .unwrap();
                    output.set_content(text);
                },
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
                    self.cursive.call_on_id(&format!("electrum_balance_{}", &coin), |textview: &mut TextView| {
                        textview.set_content(&balance)
                    });

                    dbg!(&address);

                    self.cursive.call_on_id(&format!("electrum_coin_{}", &coin), |textview: &mut TextView| {
                        textview.set_content(&address)
                    });
                },
                UiMessage::OrderbookSelectCoin(coins) => {
                    let mut sv = SelectView::<String>::new();
                    sv.add_all_str(coins);
                    let controller_tx_clone = self.controller_tx.clone();
                    sv.set_on_submit(move |siv, label: &str| {
                        controller_tx_clone.send(ControllerMessage::SelectAsk(label.into()))
                    });

                    self.cursive.add_layer(Dialog::around(sv)
                        .title("Select")
                        .button("Cancel", |siv| { siv.pop_layer(); }));
                },
                UiMessage::OrderbookUpdateAskCoinSelect(balance, address, coin) => {
                    self.cursive.call_on_id("orderbook_ask_address", |tv: &mut TextView| {
                        tv.set_content(format!("\n{}: {}", &address, &balance));
                    });

                    self.cursive.call_on_id("orderbook_ask_select_btn", |btn: &mut Button| {
                        btn.set_label(&coin);
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