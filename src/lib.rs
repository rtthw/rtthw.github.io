
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement};



#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");
    body.append_child(&val)?;

    setup_sidenav(&document)?;

    Ok(())
}



#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str);
}



// ================================================================================================



fn setup_sidenav(doc: &Document) -> Result<(), JsValue> {
    {
        let document = doc.clone();
        let open_action = Closure::<dyn Fn()>::new(move || {
            log("Opening sidenav...");
            let sidenav = document
                .get_element_by_id("sidenav")
                .expect("sidenav should exist");
            let _ = sidenav.set_attribute("style", "width:'250px'");
        });
        doc
            .get_element_by_id("sidenav-open")
            .expect("sidenav-open should exist")
            .dyn_ref::<HtmlElement>()
            .expect("sidenav-open should be an HTML element")
            .set_onclick(Some(open_action.as_ref().unchecked_ref()));
        open_action.forget();
    }
    {
        let document = doc.clone();
        let close_action = Closure::<dyn Fn()>::new(move || {
            log("Closing sidenav...");
            let sidenav = document
                .get_element_by_id("sidenav")
                .expect("sidenav should exist");
            let _ = sidenav.set_attribute("style", "width:'0px'");
        });
        doc
            .get_element_by_id("sidenav-close")
            .expect("sidenav-close should exist")
            .dyn_ref::<HtmlElement>()
            .expect("sidenav-close should be an HTML element")
            .set_onclick(Some(close_action.as_ref().unchecked_ref()));
        close_action.forget();
    }

    Ok(())
}
