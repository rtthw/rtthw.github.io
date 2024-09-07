//! SPA Routing Functionality


use eframe::egui;
use egui_router::{EguiRouter, Request, Route, TransitionConfig};

use crate::{Message, State};


pub fn setup_router(state: &mut State) -> EguiRouter<State> {
    EguiRouter::builder()
        .error_ui(|ui, state: &State, error| {
            ui.label(format!("Error: {}", error));
            if ui.button("Back").clicked() {
                state.send_message(Message::GoBack).ok();
            }
        })
        .loading_ui(|ui, _| {
            ui.label("Loading...");
            ui.spinner();
        })
        .transition(TransitionConfig::fade_up()) // .with_easing(egui_animation::easing::quad_out)
        .default_duration(0.2)
        .route("/", home)
        .route("/post/{id}", post)
        .route("/wiki/{id}", wiki)
        .default_path("/")
        .build(state)
}

fn home(_request: Request<State>) -> impl Route<State> {
    |ui: &mut egui::Ui, state: &mut State| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Home");
            ui.separator();

            ui.label("Posts:");

            link(ui, state, "Post 1", "/post/1");
            link(ui, state, "Post 2", "/post/2");

            ui.label("Wiki:");

            link(ui, state, "Article 1", "/wiki/1");
            link(ui, state, "Article 2", "/wiki/2");
        });
    }
}

fn post(request: Request<State>) -> impl Route<State> {
    let id = request.params.get("id").map(ToOwned::to_owned);

    move |ui: &mut egui::Ui, state: &mut State| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            if let Some(id) = &id {
                ui.label(format!("Post: {}", id));

                if ui.button("Back").clicked() {
                    state.send_message(Message::GoBack).ok();
                }

                ui.label("...");
            } else {
                ui.label("Post not found");
                if ui.button("Back").clicked() {
                    state.send_message(Message::GoBack).ok();
                }
            }
        });
    }
}

fn wiki(request: Request<State>) -> impl Route<State> {
    let id = request.params.get("id").map(ToOwned::to_owned);

    move |ui: &mut egui::Ui, state: &mut State| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            if let Some(id) = &id {
                ui.label(format!("Article: {}", id));

                if ui.button("Back").clicked() {
                    state.send_message(Message::GoBack).ok();
                }

                ui.label("...");
            } else {
                ui.label("Article not found");
                if ui.button("Back").clicked() {
                    state.send_message(Message::GoBack).ok();
                }
            }
        });
    }
}

fn link(ui: &mut egui::Ui, state: &State, text: &str, to: &str) {
    if ui.link(text).clicked() {
        state.send_message(Message::GoTo(to.to_string())).ok();
    }
}
