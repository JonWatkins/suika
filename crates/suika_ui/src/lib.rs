use wasm_bindgen::prelude::*;
use web_sys::{console, Document, HtmlElement, Window};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console::log_1(&"hello world from wasm".into());

    let window: Window = web_sys::window().expect("should have a Window");
    let document: Document = window.document().expect("should have a Document");

    let elements = document
        .query_selector_all("[data-console-log]")
        .expect("should query selector");

    for i in 0..elements.length() {
        let element = elements.get(i).expect("should get element");
        let element = element
            .dyn_into::<HtmlElement>()
            .expect("should be an HtmlElement");

        let message = element
            .get_attribute("data-console-log")
            .unwrap_or_else(|| "".to_string());
        let message = message.clone();

        let closure = Closure::wrap(Box::new(move || {
            console::log_1(&message.clone().into());
        }) as Box<dyn FnMut()>);

        element.set_onclick(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    Ok(())
}
