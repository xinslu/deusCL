use yew::TargetCast;
use yew::{events::KeyboardEvent, html, Component, Context, Html};
use weblog::console_log;
use code::repl;
use code::interpreter;
use yew::prelude::*;


pub enum Msg {
    Submit,
    OnInput(String),
    PrevInput
}

#[derive(Default, PartialEq, Properties)]
pub struct LineProps {
    done: bool
}


#[derive(PartialEq, Default, Debug, Clone)]
pub struct Input {
    code: String,
    result: String
}

struct Repl {
    line: i32,
    lines: Vec<Input>,
    interpreter: interpreter::Interpreter
}

impl Component for Repl {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut lines = Vec::new();
        lines.push(Input { code: "".to_string(), result: "".to_string()});
        Self { line: 0, lines: lines, interpreter: interpreter::Interpreter::new()}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                let length = self.lines.len()-1;
                let mut line = &mut self.lines[length];
                console_log!(self.line, line.code.clone());
                let result = repl::repl(line.code.clone(), &mut self.interpreter);
                console_log!(result.clone());
                line.result = result.clone();
                self.lines.push(Input { code: "".to_string(), result: "".to_string()});
                self.line += 1;
                true
            },
            Msg::PrevInput => {
                if self.line > 0 {
                    console_log!("Here");
                    let length = self.lines.len()-1;
                    self.lines[length].code = self.lines[length-1].code.clone();
                }
                true
            }
             Msg::OnInput(value) => {
                let length = self.lines.len()-1;
                let mut line = &mut self.lines[length];
                line.code = value;
                true
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeypress = ctx.link().batch_callback(|event: KeyboardEvent| {
            if event.key() == "Enter" {
                Some(Msg::Submit)
            } else if event.key() == "ArrowUp" {
                console_log!(event.key());
                Some(Msg::PrevInput)
            } else {
                console_log!(event.key());
                None
            }
        });

        html! {
            <div>
            {
                for self.lines.clone().into_iter().map(|line| {
                    html! {
                        <>
                            {
                                if line.result == "" {
                                    html! {
                                        <>
                                         <h4> {"DEUS-USER>  "}
                                            <input type="text" onkeydown={onkeypress.clone()} oninput={ ctx.link().callback(|e: web_sys::InputEvent| {
                                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                Msg::OnInput( input.value() ) })} />
                                            </h4>
                                            <h3 id="result0">{line.result.clone()}</h3>
                                        </>
                                    }

                                 } else {
                                    html! {
                                        <>
                                            <h4> {"DEUS-USER>  "}
                                                <input type="text" placeholder={line.code.clone()} readonly=true />
                                            </h4>
                                            <h3 id="result0">{line.result.clone()}</h3>
                                        </>
                                    }
                                }
                            }
                        </>
                    }
                })
            }
            </div>
        }
    }

}



fn main() {
    yew::start_app::<Repl>();
}
