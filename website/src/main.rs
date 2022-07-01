use yew::TargetCast;
use yew::{events::KeyboardEvent, html, Component, Context, Html};
use web_sys;
use weblog::console_log;
use code::repl;
use yew::prelude::*;


enum Msg {
    Submit,
    OnInput(String)
}


#[derive(PartialEq, Default, Debug, Clone)]
struct Input {
    line: i32,
    code: String,
    result: String
}

impl Component for Input {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { line: 0 , code: "".to_string(), result: "".to_string()}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
               console_log!(self.line, self.code.clone());
               let result = repl::repl(self.code.clone());
               console_log!(result.clone());
               self.result = result.clone();
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
                 <h3 id="result0">{self.result.clone()}</h3>
            </>
        }
    }
}

#[derive(Default, PartialEq, Properties)]
pub struct ListProps {
    pub children: Input,
}


enum PMsg {
    AddLine,
}

struct Repl {
    lines: Vec<Input>,
}

impl Component for Repl {
    type Message = PMsg;
    type Properties = ListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { lines: Vec::new()}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                 {for self.lines.clone().into_iter().map(|line| {
                    line.view()
                 })}
            </div>
        }
    }

}



fn main() {
    yew::start_app::<Input>();
}
