use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, Document, HtmlElement, Request, RequestInit, Response, Window};

// Enum to represent HTTP methods
enum HttpMethod {
    Get,
    Post,
}

impl HttpMethod {
    // Convert HttpMethod to string
    fn as_str(&self) -> &str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
        }
    }
}

// Enum to represent event triggers
enum EventTrigger {
    Click,
    Submit,
    Change,
    Custom(String),
}

impl EventTrigger {
    // Convert EventTrigger to string
    fn as_str(&self) -> &str {
        match self {
            EventTrigger::Click => "click",
            EventTrigger::Submit => "submit",
            EventTrigger::Change => "change",
            EventTrigger::Custom(event) => event,
        }
    }

    // Create EventTrigger from string
    fn from_str(event: &str) -> Self {
        match event {
            "click" => EventTrigger::Click,
            "submit" => EventTrigger::Submit,
            "change" => EventTrigger::Change,
            custom => EventTrigger::Custom(custom.to_string()),
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console::log_1(&"hello world from wasm".into());

    let window: Window = web_sys::window().expect("should have a Window");
    let document: Document = window.document().expect("should have a Document");

    let suika_ui = SuikaUI::new(window, document);
    suika_ui.init()?;

    Ok(())
}

// Define the SuikaUI struct
struct SuikaUI {
    window: Window,
    document: Document,
}

impl SuikaUI {
    // Constructor for SuikaUI
    pub fn new(window: Window, document: Document) -> Self {
        SuikaUI { window, document }
    }

    // Initialize the framework by adding event listeners
    pub fn init(&self) -> Result<(), JsValue> {
        let elements = self.query_elements("[data-sx-get], [data-sx-post]")?;
        self.add_event_listeners(elements)?;
        Ok(())
    }

    // Query elements with specified selector
    fn query_elements(&self, selector: &str) -> Result<web_sys::NodeList, JsValue> {
        let elements = self.document.query_selector_all(selector)?;
        console::log_1(
            &format!(
                "Found {} elements with {} attributes",
                elements.length(),
                selector
            )
            .into(),
        );
        Ok(elements)
    }

    // Add event listeners to elements
    fn add_event_listeners(&self, elements: web_sys::NodeList) -> Result<(), JsValue> {
        for i in 0..elements.length() {
            let element = elements.item(i).expect("should get element");
            let element = element
                .dyn_into::<HtmlElement>()
                .expect("should be an HtmlElement");

            let method = self.get_http_method(&element)?;
            let url = self.get_attribute(
                &element,
                &format!("data-sx-{}", method.as_str().to_lowercase()),
            )?;
            let target_selector = self.get_attribute(&element, "data-sx-target")?;
            let trigger_event = EventTrigger::from_str(
                &self
                    .get_attribute(&element, "data-sx-trigger")
                    .unwrap_or_else(|_| "click".to_string()),
            );

            self.add_event_listener(element, method, url, target_selector, trigger_event)?;
        }
        Ok(())
    }

    // Get HTTP method from element
    fn get_http_method(&self, element: &HtmlElement) -> Result<HttpMethod, JsValue> {
        if element.has_attribute("data-sx-get") {
            Ok(HttpMethod::Get)
        } else if element.has_attribute("data-sx-post") {
            Ok(HttpMethod::Post)
        } else {
            Err(JsValue::from_str(
                "Element does not have data-sx-get or data-sx-post attribute",
            ))
        }
    }

    // Get attribute value from element
    fn get_attribute(&self, element: &HtmlElement, attribute: &str) -> Result<String, JsValue> {
        element.get_attribute(attribute).ok_or_else(|| {
            JsValue::from_str(&format!("Element does not have {} attribute", attribute))
        })
    }

    // Add event listener to element
    fn add_event_listener(
        &self,
        element: HtmlElement,
        method: HttpMethod,
        url: String,
        target_selector: String,
        trigger_event: EventTrigger,
    ) -> Result<(), JsValue> {
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            event.prevent_default(); // Prevent default behavior (e.g., form submission)

            let url = url.clone();
            let target_selector = target_selector.clone();
            let method = method.as_str().to_string();

            wasm_bindgen_futures::spawn_local(async move {
                if let Err(e) = SuikaUI::handle_request(&url, &target_selector, &method).await {
                    console::log_1(&format!("Error handling {} request: {:?}", method, e).into());
                }
            });
        }) as Box<dyn FnMut(_)>);

        element.add_event_listener_with_callback(
            trigger_event.as_str(),
            closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
        Ok(())
    }

    // Handle requests (GET, POST, etc.)
    async fn handle_request(url: &str, target_selector: &str, method: &str) -> Result<(), JsValue> {
        let window = web_sys::window().expect("should have a window");
        let document = window.document().expect("should have a document");

        let opts = RequestInit::new();
        opts.set_method(method);

        let request = Request::new_with_str_and_init(url, &opts)?;
        let response = JsFuture::from(window.fetch_with_request(&request))
            .await?
            .dyn_into::<Response>()?;

        if response.ok() {
            let text = JsFuture::from(response.text()?)
                .await?
                .as_string()
                .unwrap_or_default();
            if let Some(target_element) = document.query_selector(target_selector)? {
                target_element.set_inner_html(&text);
            }
        } else {
            console::log_1(&format!("Failed to fetch data: {:?}", response.status_text()).into());
        }

        Ok(())
    }
}
