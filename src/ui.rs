use cursive::Cursive;
use std::sync::mpsc;
use crate::controller::ControllerMessage;
use cursive::event::Key;
use cursive::views::{TextArea, Dialog, TextView};
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
    StartMainLayer
}

impl Ui {
    pub fn new(controller_tx: mpsc::Sender<ControllerMessage>) -> Self {
        let (ui_tx, ui_rx) = mpsc::channel::<UiMessage>();

        let mut ui = Ui {
            cursive: Cursive::default(),
            ui_tx,
            ui_rx,
            controller_tx
        };

        // Create a view tree with a TextArea for input, and a
        // TextView for output.


        // Create all views here, send a controller_tx along with it.
        // whenever a view needs updating, send a message to controller, which sends back
        // the requested information

        let controller_tx_clone = ui.controller_tx.clone();
        ui.cursive.add_layer(LoginView::new(controller_tx_clone.clone()));

        // Configure a callback
        ui.cursive.add_global_callback(Key::Esc, move |c| {
            // When the user presses Escape, send an
            // UpdatedInputAvailable message to the controller.
            let input = c.find_id::<TextArea>("input").unwrap();
            let text = input.get_content().to_owned();
            controller_tx_clone.send(
                ControllerMessage::FetchBalance(text))
                .unwrap();
        });
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
                    let main_view = crate::main_v::create();
                    self.cursive.pop_layer();
                    self.cursive.add_layer(main_view);
                }
            }
        }

        // Step the UI
        self.cursive.step();

        true
    }
}