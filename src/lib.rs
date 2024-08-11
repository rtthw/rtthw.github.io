

use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;



#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");
    body.append_child(&val)?;

    let btn_action = Closure::<dyn FnMut()>::new(move || {
        log("Button success!");
    });
    document
        .get_element_by_id("test-button")
        .expect("should have #test-button on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#test-button should be an `HtmlElement`")
        .set_onmousedown(Some(btn_action.as_ref().unchecked_ref()));
    btn_action.forget();

    Ok(())
}



#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str);
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
