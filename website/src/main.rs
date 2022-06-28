use wasm_bindgen::JsCast;
use yew::{function_component, html, FocusEvent};
use web_sys::{EventTarget, HtmlFormElement};
use log::info;

#[function_component(LineInput)]
fn form_prompt() -> Html {
    let onsubmit = |event: FocusEvent| {
        event.prevent_default();
        let target: Option<EventTarget> = event.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlFormElement>().ok());
        if let Some(some_value) = input {
            info!("{:?}", some_value.target());
        }
    };
    html! {
        <>
        <form autocomplete="off" onsubmit={onsubmit}>
            <h4>{"DEUSCL-USER> "}
                <input id="code0" />
                <input type="submit" style="display: none" />
            </h4>
        </form>
        <h3 id="result0"></h3>
        </>
    }
}

// Then somewhere else you can use the component inside `html!`
#[function_component(App)]
fn app() -> Html {
    html! { <LineInput /> }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
