


use dioxus::prelude::*;



fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! { Router::<Route> {} }
}



#[derive(Routable, Clone)]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[nest("/blog")]
            #[layout(BlogIndex)]
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
fn NavBar() -> Element {
    rsx! {
        nav {
            ul {
                li {
                    Link { to: Route::Home {}, "Home" }
                }
                li {
                    Link { to: Route::Blog {}, "Blog" }
                }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn BlogIndex() -> Element {
    rsx! {
        h1 { "Blog" }
        Outlet::<Route> {}
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
