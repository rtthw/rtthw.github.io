use leptos::*;
use leptos_router::*;



fn main() {
    mount_to_body(|| view! { <App/> });
}



#[component]
pub fn app() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    leptos_meta::provide_meta_context();
    provide_context(GlobalState::new());

    view! {
        <Router>

            <nav>
            </nav>

            <header></header>

            <main class="container">
                <SideNav/>
                <Routes>
                    <Route path="" view=Home/>
                    <Route path="blog" view=Blog/>
                    // <Route path="blog/:id" view=Post/>
                    <Route path="about" view=About/>
                    <Route path="*" view=PageNotFound/>
                </Routes>
            </main>

            <footer></footer>

        </Router>
    }
}

#[derive(Copy, Clone, Debug)]
struct GlobalState {
    show_sidenav: RwSignal<bool>,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            show_sidenav: create_rw_signal(false),
        }
    }
}



// ================================================================================================



#[component]
pub fn side_nav() -> impl IntoView {
    let state = use_context::<GlobalState>().unwrap();
    provide_context(state.clone());
    let hidden = !state.show_sidenav.get();
    view! {
        <div id="sidenav" class:sidenav_hidden=hidden class:sidenav_shown=state.show_sidenav>
            <button 
                id="sidenav-close" 
                on:click=move |_| state.show_sidenav.update(|v| *v = !*v)
            >
                X
            </button>
            <A exact=true href="/">"Home"</A>
            <A href="blog">"Blog"</A>
            <A href="about">"About"</A>
        </div>
    }
}

#[component]
pub fn home() -> impl IntoView {
    let state = use_context::<GlobalState>().unwrap();
    provide_context(state.clone());
    view! {
        <p>Home</p>
        <button 
            on:click=move |_| state.show_sidenav.update(|v| *v = !*v)
        >
            "X"
        </button>
    }
}

#[component]
pub fn blog() -> impl IntoView {
    view! {
        <p>Blog</p>
    }
}

#[component]
pub fn about() -> impl IntoView {
    view! {
        <p>About</p>
    }
}

#[component]
pub fn page_not_found() -> impl IntoView {
    view! {
        <p>Oops!</p>
    }
}
