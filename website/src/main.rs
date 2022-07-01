use yew::TargetCast;
use yew::{events::KeyboardEvent, html, Component, Context, Html};
use web_sys;
use weblog::{console_log};

enum Msg {
    Submit,
    OnInput(String)
}

struct Input {
    line: i32,
    code: String
}

impl Component for Input {

    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { line: 0 , code: "".to_string()}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
               console_log!("Hello world!", self.line, self.code.clone());
                true
            },
            Msg::OnInput(value) => {
                self.code = value;
                true
            }
        };
        true
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeypress = ctx.link().batch_callback(|event: KeyboardEvent| {
            if event.key() == "Enter" {
                Some(Msg::Submit)
            } else {
                None
            }
        });

        html! {
            <>
                <h4> {"DEUS-USER>  "}
                    <input type="text" {onkeypress} oninput={ ctx.link().callback(|e: web_sys::InputEvent| {
                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                    Msg::OnInput( input.value() ) })} />
                </h4>
            </>
        }
    }
}


fn main() {
    yew::start_app::<Input>();
}
