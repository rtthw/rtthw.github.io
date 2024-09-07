


mod routing;

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
}

impl eframe::App for Website {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.state.messages.is_empty() {
            let messages: Vec<Message> = self.state.messages.drain(..).collect();
            for msg in messages {
                match msg {
                    Message::GoTo(route) => {
                        self.router.navigate(&mut self.state, route).ok();
                    }
                    Message::GoBack => {
                        self.router.back().ok();
                    }
                }
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("Coming Soon...");
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

        let mut state = State {
            messages: vec![],
        };
        let router = routing::setup_router(&mut state);

        Self {
            state,
            router,
        }
    }
}

pub struct State {
    messages: Vec<Message>,
}

impl State {
    pub fn send_message(&mut self, msg: Message) {
        self.messages.push(msg)
    }
}

pub enum Message {
    GoTo(String),
    GoBack,
}
