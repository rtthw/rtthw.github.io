


use dioxus::prelude::*;



const RAW_SITE_DATA_URL: &str = "https://raw.githubusercontent.com/rtthw/data/master/site-data";



// ================================================================================================



fn main() {
    launch(app);
}

fn app() -> Element { rsx! { Router::<Route> {} } }

#[derive(Routable, Clone)]
enum Route {
    #[layout(MainLayout)]
        #[route("/")]
        Home {},
        #[route("/about")]
        About {},
        #[nest("/projects")]
            #[layout(ProjectsLayout)]
                #[route("/")]
                Projects {},
                #[route("/:name")]
                Project { name: String },
            #[end_layout]
        #[end_nest]
        #[nest("/blog")]
            #[layout(BlogLayout)]
                #[route("/")]
                Blog {},
                #[route("/post/:name")]
                Post { name: String },
            #[end_layout]
        #[end_nest]
        #[nest("/wiki")]
            #[layout(WikiLayout)]
                #[route("/")]
                Wiki {},
                #[route("/:name")]
                Article { name: String },
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
        // Side bar.
        div {
            class: "w3-sidebar w3-margin w3-animate-left w3-transparent",
            display: nav_display,
            z_index: "5",

            div {
                class: "w3-card w3-round w3-white",

                div {
                    class: "w3-container w3-padding",
                    
                    h1 { class: "w3-xxxlarge w3-center w3-monospace w3-text-blue-grey", "rtthw" },
    
                    Link {
                        class: "w3-xlarge", 
                        to: Route::Home {}, 
                        onclick: move |_| nav_display.set("none"),

                        "Home" 
                    },
                    br {}
                    Link {
                        class: "w3-xlarge", 
                        to: Route::About {}, 
                        onclick: move |_| nav_display.set("none"),

                        "About" 
                    },
                }
            }
            div {
                class: "w3-card w3-round w3-white w3-margin-top",

                div {
                    class: "w3-container w3-padding",

                    Link {
                        class: "w3-xlarge", 
                        to: Route::Projects {}, 
                        onclick: move |_| nav_display.set("none"),

                        "Projects"
                    },
                    br {}
                    Link {
                        class: "w3-xlarge", 
                        to: Route::Blog {}, 
                        onclick: move |_| nav_display.set("none"),

                        "Blog"
                    },
                    br {}
                    Link {
                        class: "w3-xlarge", 
                        to: Route::Wiki {}, 
                        onclick: move |_| nav_display.set("none"),

                        "Wiki"
                    },
                }
            }
        }

        // Opacity overlay.
        div {
            class: "w3-overlay w3-animate-opacity",
            display: nav_display,
            cursor: "pointer",
            onmousedown: move |_| nav_display.set("none"),
        }

        // Menu bar.
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

        // Content.
        div {
            class: "w3-content",
            margin_top: "70px",

            Outlet::<Route> {}
        }
    }
}



// ================================================================================================



#[component]
fn Home() -> Element {
    rsx! { h1 { "Home" } }
}

#[component]
fn About() -> Element {
    rsx! { h1 { "About" } }
}

#[component]
fn BlogLayout() -> Element {
    rsx! {
        h1 { "Blog" }

        div {
            class: "w3-card w3-white w3-round w3-margin",

            div {
                class: "w3-container",

                Outlet::<Route> {}
            }
        }
    }
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
fn ProjectsLayout() -> Element {
    rsx! {
        div {
            class: "w3-card w3-white w3-round w3-margin",

            div {
                class: "w3-container",

                Outlet::<Route> {}
            }
        }
    }
}

#[component]
fn Projects() -> Element {
    rsx! {
        h1 { "Projects" }

        h3 { "Applications" }
        ul {
            li {
                Link {
                    class: "w3-xlarge", 
                    to: Route::Project {
                        name: "error".into(),
                    },
                    "UNNAMED: A terminal file manager"
                }
            }
        }
        h3 { "Development Tools" }
        ul {
            li {
                Link {
                    class: "w3-xlarge", 
                    to: Route::Project {
                        name: "dreg".into(),
                    },
                    "Dreg: A simple TUI library written in Rust"
                }
            }
        }
    }
}

#[component]
fn Project(name: String) -> Element {
    let url = format!("{RAW_SITE_DATA_URL}/projects/{}.md", &name);
    let mut future = use_resource(move || {
        let value = url.clone();
        async move {
            reqwest::get(value)
                .await
                .unwrap()
                .bytes()
                .await
        }
    });

    match &*future.read_unchecked() {
        Some(Ok(response)) => {
            let content = String::from_utf8(response.to_vec()).unwrap();
            rsx! {
                h1 { class: "w3-xxxlarge w3-center w3-monospace w3-text-blue-grey", "{name}" }

                div {
                    class: "container is-fluid",
                    Markdown {
                        content: content,
                    }
                }
            }
        }
        Some(Err(_)) => rsx! {
            div { "Loading page failed" }
            a {
                class: "w3-button w3-right w3-hover-white w3-theme",
                title: "Retry Fetch",
                onmousedown: move |_| future.restart(),

                i { class: "fa fa-refresh" }
            }
        },
        None => rsx! { div { "Loading page..." } },
    }
}

#[component]
fn WikiLayout() -> Element {
    rsx! {
        div {
            class: "w3-card w3-white w3-round w3-margin",

            div {
                class: "w3-container",

                Outlet::<Route> {}
            }
        }
    }
}

#[component]
fn Wiki() -> Element {
    rsx! {
        h1 { "Wiki" }
    }
}

#[component]
fn Article(name: String) -> Element {
    let url = format!("{RAW_SITE_DATA_URL}/wiki/{}.md", &name);
    let mut future = use_resource(move || {
        let value = url.clone();
        async move {
            reqwest::get(value)
                .await
                .unwrap()
                .bytes()
                .await
        }
    });

    match &*future.read_unchecked() {
        Some(Ok(response)) => {
            let content = String::from_utf8(response.to_vec()).unwrap();
            rsx! {
                h1 { class: "w3-xxxlarge w3-center w3-monospace w3-text-blue-grey", "{name}" }

                div {
                    class: "container is-fluid",
                    Markdown {
                        content: content,
                    }
                }
            }
        }
        Some(Err(_)) => rsx! {
            div { "Loading page failed" }
            a {
                class: "w3-button w3-right w3-hover-white w3-theme",
                title: "Retry Fetch",
                onmousedown: move |_| future.restart(),

                i { class: "fa fa-refresh" }
            }
        },
        None => rsx! { div { "Loading article..." } },
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "The page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}



// ================================================================================================



#[derive(Props, Clone, PartialEq)]
pub struct MarkdownProps {
    #[props(default)]
    id: Signal<String>,
    #[props(default)]
    class: Signal<String>,

    content: ReadOnlySignal<String>,
}

/// Render some text as markdown.
#[component]
pub fn Markdown(props: MarkdownProps) -> Element {
    let content = &*props.content.read();
    let parser = pulldown_cmark::Parser::new(content);

    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);

    rsx! {
        div {
            id: "{&*props.id.read()}",
            class: "{&*props.class.read()}",
            dangerous_inner_html: "{html_buf}"
        }
    }
}
