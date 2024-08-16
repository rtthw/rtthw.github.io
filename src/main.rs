


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
            #[route("/")]
            Projects {},
            #[route("/:..route")]
            Project { route: Vec<String> },
        #[end_nest]
        #[nest("/blog")]
            #[layout(BlogLayout)]
                #[route("/")]
                Blog {},
                #[route("/post/:..route")]
                Post { route: Vec<String> },
            #[end_layout]
        #[end_nest]
        #[nest("/wiki")]
            #[layout(WikiLayout)]
                #[route("/")]
                Wiki {},
                #[route("/:..route")]
                Article { route: Vec<String> },
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
                        route: vec!["how-i-made-this-site".into()],
                    },
                    "How I Made This Site"
                }
            }
            // li {
            //     Link {
            //         to: Route::Post {
            //             route: vec!["Blog post 2".into()],
            //         },
            //         "Read the second blog post"
            //     }
            // }
        }
    }
}

#[component]
fn Post(route: Vec<String>) -> Element {
    let route_info = resolve_route_info("blog", &route);

    let url = route_info.url.clone();
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
                h1 {
                    class: "w3-xxxlarge w3-center w3-monospace w3-text-blue-grey",
                    "{route_info.real_name}"
                }

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
fn Projects() -> Element {
    rsx! {
        h1 { "Projects" }

        h3 { "Applications" }
        ul {
            li {
                Link {
                    class: "w3-xlarge", 
                    to: Route::Project {
                        route: vec!["error".into()],
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
                        route: vec!["dreg".into()],
                    },
                    "Dreg: A simple TUI library written in Rust"
                }
            }
        }
    }
}

#[component]
fn Project(route: Vec<String>) -> Element {
    let route_info = resolve_route_info("projects", &route);

    let url = route_info.url.clone();
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
                h1 {
                    class: "w3-xxxlarge w3-center w3-monospace w3-text-blue-grey",
                    "{route_info.real_name}"
                }

                div {
                    class: "w3-card w3-white w3-round w3-margin",
        
                    div {
                        class: "w3-container",
        
                        Markdown {
                            content: content,
                        }
                    }
                }
            }
        }
        Some(Err(_)) => rsx! {
            h3 { "Loading failed" }
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
fn Article(route: Vec<String>) -> Element {
    let route_info = resolve_route_info("wiki", &route);

    let url = route_info.url.clone();
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
                h1 {
                    class: "w3-xxxlarge w3-center w3-monospace w3-text-blue-grey",
                    "{route_info.real_name}"
                }

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


pub struct RouteInfo {
    pub name: String,
    pub real_name: String,
    pub url: String,
}

fn resolve_route_info(prefix: &str, route: &Vec<String>) -> RouteInfo {
    let name = route.last().cloned().unwrap_or(String::new());
    let file_name = if name.ends_with(".md") {
        name.clone()
    } else {
        format!("{name}.md")
    };
    let url = if route.len() > 1 {
        let path_before_name = route.iter().cloned()
            .take(route.len() - 1)
            .fold(String::new(), |mut acc, seg| {
                acc.push_str(&format!("{seg}/"));
                acc
            });
        format!("{RAW_SITE_DATA_URL}/{prefix}/{}{}", path_before_name, &file_name)
    } else {
        format!("{RAW_SITE_DATA_URL}/{prefix}/{}", &file_name)
    };
    let real_name = name.strip_suffix(".md").unwrap_or(&name).to_string();

    RouteInfo {
        name,
        real_name,
        url,
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
