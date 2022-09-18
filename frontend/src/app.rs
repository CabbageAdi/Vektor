use yew::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use crate::components::text_input::*;

pub enum AppMsg {
    Increment,
    SetIncrement(String)
}

pub struct App {
    value: i32,
    increment_value: String
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { value: 0, increment_value: String::new() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::Increment => self.value += self.increment_value.parse::<i32>().unwrap_or_default(),
            AppMsg::SetIncrement(val) => self.increment_value = val
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{"Hello World"}</p>
                <p>{self.value}</p>

                <button onclick={ctx.link().callback(|_| AppMsg::Increment)}>{"increment"}</button>
                <TextInput on_change={ctx.link().callback(AppMsg::SetIncrement)} value={self.increment_value.clone()} />
            </div>
        }
    }
}