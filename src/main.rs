use leptos::*;
use leptos_router::*;



fn main() {
    mount_to_body(|| view! { <App/> });
}



#[component]
pub fn app() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    leptos_meta::provide_meta_context();

    view! {
        <div id="root">
            <Router>
                <main class="container">
                    <Routes>
                        <Route path="" view=Home/>
                        <Route path="blog" view=Blog/>
                        // <Route path="blog/:id" view=Post/>
                        <Route path="about" view=About/>
                        <Route path="*" view=PageNotFound/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}



// ================================================================================================



#[component]
pub fn home() -> impl IntoView {
    view! {
        <p>Home</p>
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
