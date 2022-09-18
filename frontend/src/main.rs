mod app;
mod components;

use yew::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use crate::app::App;

fn main() {
    yew::start_app::<App>();
}
