


mod routing;

use anyhow::Result;
use eframe::egui::{self, Color32};




// ================================================================================================



// const RAW_SITE_DATA_URL: &str = "https://raw.githubusercontent.com/rtthw/data/master/site-data";


#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    eframe::run_native(
        "rtthw.github.io",
        Default::default(),
        Box::new(|cc| Ok(Box::new(Website::new(cc)))),
    )
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new().start(
            "canvas",
            eframe::WebOptions::default(),
            Box::new(|cc| Ok(Box::new(Website::new(cc)))),
        )
        .await
        .expect("failed to start eframe");
    });
}



// ================================================================================================



struct Website {
    state: State,
    router: egui_router::EguiRouter<State>,
    msg_recv: std::sync::mpsc::Receiver<Message>,
}

impl eframe::App for Website {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(msg) = self.msg_recv.try_recv() {
            match msg {
                Message::GoTo(route) => {
                    self.router.navigate(&mut self.state, route).ok();
                }
                Message::GoBack => {
                    self.router.back().ok();
                }
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                self.router.ui(ui, &mut self.state)
            });
        });
    }
}

impl Website {
    pub fn new(cc: &eframe::CreationContext) -> Self {

        let mut visuals = egui::Visuals::dark();
        visuals.button_frame = false;
        visuals.interact_cursor = Some(egui::CursorIcon::PointingHand);
        visuals.extreme_bg_color = Color32::from_hex("#1e1f22").unwrap();
        visuals.panel_fill = Color32::from_hex("#1e1f22").unwrap();
        cc.egui_ctx.set_visuals(visuals);

        let (mut state, msg_recv) = State::new();
        let router = routing::setup_router(&mut state);

        Self {
            state,
            router,
            msg_recv,
        }
    }
}

pub struct State {
    msg_send: std::sync::mpsc::Sender<Message>,
}

impl State {
    fn new() -> (Self, std::sync::mpsc::Receiver<Message>) {
        let (msg_send, msg_recv) = std::sync::mpsc::channel();

        (
            Self {
                msg_send,
            },
            msg_recv,
        )
    }

    pub fn send_message(&self, msg: Message) -> Result<()> {
        self.msg_send.send(msg)?;
        Ok(())
    }
}

pub enum Message {
    GoTo(String),
    GoBack,
}
