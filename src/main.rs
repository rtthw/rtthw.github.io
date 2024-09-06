


use eframe::egui;



// const RAW_SITE_DATA_URL: &str = "https://raw.githubusercontent.com/rtthw/data/master/site-data";



// ================================================================================================



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



struct Website {}

impl eframe::App for Website {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading("Coming Soon...");
            });
        });
    }
}

impl Website {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {}
    }
}
