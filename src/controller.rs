use std::sync::mpsc;
use crate::ui::{UiMessage, Ui};
use std::{thread, fs};
use std::process::Command;
use std::time::Duration;
use std::fs::File;
use serde::{Serialize, Deserialize};

pub struct Controller {
    rx: mpsc::Receiver<ControllerMessage>,
    ui: Ui,
    client: mmapi::Client
}

pub enum ControllerMessage {
    UpdatedInputAvailable(String),
    FetchBalance(String),
    StartMainLayer(String),
    ElectrumActivate(String)
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
        })
    }

    pub fn run(&mut self) {
        while self.ui.step() {
            // on each step, clear the message queue that the controller receives
            while let Some(message) = self.rx.try_iter().next() {
                match message {
                    ControllerMessage::UpdatedInputAvailable(text) => {
                        self.ui
                            .ui_tx
                            .send(UiMessage::UpdateOutput(text))
                            .unwrap();
                    },
                    ControllerMessage::FetchBalance(coin) => {
                        let balance = self.client.balance(&coin).unwrap();
                        self.ui
                            .ui_tx
                            .send(UiMessage::Balance(balance.balance.unwrap()))
                            .unwrap();
                    },
                    ControllerMessage::StartMainLayer(passphrase) => {
                        let userhome = dirs::home_dir().expect("Unable to get userhome");
                        let mm2_json = Mm2Json::create(
                            &self.client.get_userpass(),
                            passphrase.as_str(),
                            userhome.to_str().unwrap()
                        );

                        mm2_json.create_mm2_json();

//                        thread::spawn( move || {
//                            let _mm2client =
//                                Command::new("./marketmaker")
//                                    .spawn()
//                                    .expect("Failed to start");
//                        });

//                        thread::sleep(Duration::from_secs(1));
                        std::fs::remove_file("MM2.json");

                        self.ui
                            .ui_tx
                            .send(UiMessage::StartMainLayer)
                            .unwrap();
                    },
                    ControllerMessage::ElectrumActivate(coin) => {
                        let electrum = self.client.electrum(&coin, true).unwrap();

                        if let Some(error) = electrum.error {
                            // tell the UI to show the error
                        } else {
                            self.ui
                                .ui_tx
                                .send(UiMessage::ElectrumStarted((electrum.coin.unwrap(), electrum.address.unwrap(), electrum.balance.unwrap())))
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