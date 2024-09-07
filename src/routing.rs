//! SPA Routing Functionality


use eframe::egui;
use egui_router::{EguiRouter, Request, Route, TransitionConfig};

use crate::{Message, State};


pub fn setup_router(state: &mut State) -> EguiRouter<State> {
    EguiRouter::builder()
        .transition(TransitionConfig::fade_up()) // .with_easing(egui_animation::easing::quad_out)
        .default_duration(0.2)
        .route("/", home)
        .route("/post/{id}", post)
        .default_path("/")
        .build(state)
}

fn home(_request: Request<State>) -> impl Route<State> {
    |ui: &mut egui::Ui, state: &mut State| {
        background(ui, ui.style().visuals.faint_bg_color, |ui| {
            ui.heading("Home!");

            ui.label("Navigate to post:");

            if ui.link("Post 1").clicked() {
                state.send_message(Message::GoTo("/post/1".to_string()));
            }

            if ui.link("Post 2").clicked() {
                state.send_message(Message::GoTo("/post/2".to_string()));
            }

            if ui.link("Invalid Post").clicked() {
                state.send_message(Message::GoTo("/post/".to_string()));
            }
        });
    }
}

fn post(request: Request<State>) -> impl Route<State> {
    let id = request.params.get("id").map(ToOwned::to_owned);

    move |ui: &mut egui::Ui, state: &mut State| {
        background(ui, ui.style().visuals.extreme_bg_color, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(id) = &id {
                    ui.label(format!("Post: {}", id));

                    if ui.button("Back").clicked() {
                        state.send_message(Message::GoBack);
                    }

                    ui.label("...");
                } else {
                    ui.label("Post not found");
                    if ui.button("Back").clicked() {
                        state.send_message(Message::GoBack);
                    }
                }
            });
        });
    }
}

fn background(ui: &mut egui::Ui, color: egui::Color32, content: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none().fill(color).inner_margin(17.0).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.set_height(ui.available_height());
        content(ui);
    });
}
