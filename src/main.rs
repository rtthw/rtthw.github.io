


use dioxus::prelude::*;



fn main() {
    launch(app);
}

fn app() -> Element { rsx! { Router::<Route> {} } }



#[derive(Routable, Clone)]
enum Route {
    #[layout(MainLayout)]
        #[route("/")]
        Home {},
        #[nest("/blog")]
            #[layout(BlogLayout)]
                #[route("/")]
                Blog {},
                #[route("/post/:name")]
                Post { name: String },
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn MainLayout() -> Element {
    let mut nav_display = use_signal(|| "none");

    rsx! {
        div {
            class: "w3-sidebar w3-bar-block w3-margin w3-animate-left",
            display: nav_display,
            z_index: "5",

            div {
                class: "w3-container w3-card",

                h1 { "rtthw" },

                Link { to: Route::Home {}, "Home" },
                Link { to: Route::Blog {}, "Blog" },
            }
        }

        div {
            class: "w3-overlay w3-animate-opacity",
            display: nav_display,
            cursor: "pointer",
            onmousedown: move |_| nav_display.set("none"),
        }

        div {
            class: "w3-top",

            div {
                class: "w3-bar w3-theme-d2 w3-left-align w3-large",

                a {
                    class: "w3-bar-item w3-button w3-padding-large w3-hover-white w3-large w3-theme-d2",
                    onmousedown: move |_| nav_display.set("block"),

                    i { class: "fa fa-bars" }
                }

                Link {
                    class: "w3-bar-item w3-button w3-padding-large w3-theme-d4",
                    to: Route::Home {}, 
                    i { class: "fa fa-home w3-margin-right" },

                    "Home"
                }
            }
        }

        div {
            class: "w3-content",
            margin_top: "70px",

            Outlet::<Route> {}
        }
    }
}

#[component]
fn BlogLayout() -> Element {
    rsx! {
        div {
            class: "w3-content w3-margin", 

            div {
                class: "w3-container w3-card w3-white w3-margin",

                Outlet::<Route> {}
            }
        }
    }
}


#[component]
fn Home() -> Element {
    rsx! { h1 { "Home" } }
}

#[component]
fn Blog() -> Element {
    rsx! {
        h2 { "Choose a post" }
        ul {
            li {
                Link {
                    to: Route::Post {
                        name: "Blog post 1".into(),
                    },
                    "Read the first blog post"
                }
            }
            li {
                Link {
                    to: Route::Post {
                        name: "Blog post 2".into(),
                    },
                    "Read the second blog post"
                }
            }
        }
    }
}

#[component]
fn Post(name: String) -> Element {
    rsx! { h2 { "Blog Post: {name}" } }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "The page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
