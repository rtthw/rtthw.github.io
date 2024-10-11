use eframe::egui;




#[cfg(not(target_arch = "wasm32"))]
fn main() { todo!("native implementation") }

#[cfg(target_arch = "wasm32")]
fn main() {
    use wasm_bindgen::JsCast;
    let web_options = eframe::WebOptions::default();
    let element = eframe::web_sys::window()
        .expect("failed to get window")
        .document()
        .expect("failed to get document")
        .get_element_by_id("canvas")
        .expect("failed to get canvas element")
        .dyn_into::<eframe::web_sys::HtmlCanvasElement>()
        .unwrap();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                element,
                web_options,
                Box::new(|cc| Ok(Box::new(Program::new(cc)))),
            )
            .await
            .expect("failed to start eframe");
    });
}



struct Program {
    router: egui_router::EguiRouter<State>,
    state: State,
}

impl eframe::App for Program {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none()
                .fill(egui::Color32::from_hex("#1e1f22").unwrap()))
            .show(ctx, |ui| {
                self.router.ui(ui, &mut self.state);
            });
    }
}

impl Program {
    fn new(_cc: &eframe::CreationContext) -> Self {
        let mut state = State {};
        let router = egui_router::EguiRouter::builder()
            .history({
                #[cfg(target_arch = "wasm32")]
                let history = egui_router::history::BrowserHistory::new(Some("".to_string()));
                #[cfg(not(target_arch = "wasm32"))]
                let history = egui_router::history::DefaultHistory::default();
                history
            })
            .transition(egui_router::TransitionConfig::fade())
            .default_duration(0.3)
            .default_path("/")
            .route("/", home_route)
            .route("/dreg/{slug}", dreg_route)
            .build(&mut state);


        Self {
            router,
            state,
        }
    }
}



pub struct State {}



pub fn home_route(_req: egui_router::Request<State>) -> impl egui_router::Route<State> {
    move |ui: &mut egui::Ui, _state: &mut State| {
        ui.centered_and_justified(|ui| {
            ui.heading("Home");
        });
    }
}

pub fn dreg_route(req: egui_router::Request<State>) -> impl egui_router::Route<State> {
    let slug = req.params.get("slug").unwrap();
    match slug {
        "something" => {
            move |ui: &mut egui::Ui, _state: &mut State| {
                ui.centered_and_justified(|ui| {
                    ui.heading("dreg/something");
                });
            }
        }
        "nothing" => {
            move |ui: &mut egui::Ui, _state: &mut State| {
                ui.centered_and_justified(|ui| {
                    ui.heading("dreg/nothing");
                });
            }
        }
        _ => {
            move |ui: &mut egui::Ui, _state: &mut State| {
                ui.centered_and_justified(|ui| {
                    ui.heading("dreg");
                });
            }
        }
    }
}
