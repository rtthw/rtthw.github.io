


use dioxus::prelude::*;



fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! { Router::<Route> {} }
}



#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog")]
    Blog {},
    // PageNotFound is a catch all route that will match any route and placing the matched segments in the route field
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}



#[component]
fn Home() -> Element {
    rsx! { h1 { "Home" } }
}

#[component]
fn Blog() -> Element {
    rsx! { h1 { "Blog" } }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "The page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
