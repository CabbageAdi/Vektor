use raylib::prelude::*;
use crate::CanvasData;

pub trait Button {
    fn run(&self, d: &mut RaylibDrawHandle, canvas_data: &mut CanvasData, canvas: &mut RenderTexture2D, ui: &mut UI);
}

pub trait ButtonState {
    //todo: make responsive somehow
    fn get_size(&mut self) -> (i32, i32);
    //temporary hopefully
    fn get_start(&mut self) -> (i32, i32);
    fn get_text(&mut self) -> String;
    fn enabled(&mut self) -> bool;
    fn set_enabled(&mut self, enable: bool);

    fn get_behavior(&self) -> Box<dyn Button>;
}

pub trait Panel {
    //todo: make responsive somehow
    fn get_size(&mut self) -> (i32, i32);
    //temporary hopefully
    fn get_start(&mut self) -> (i32, i32);
    fn enabled(&mut self) -> bool;
    fn set_enabled(&mut self, enable: bool);
}

pub struct UI {
    pub buttons: Vec<Box<dyn ButtonState>>,
    pub panels: Vec<Box<dyn Panel>>,
    pub margin_distance: i32
}

impl UI {
    pub fn add_button(&mut self, button: impl ButtonState + 'static) {
        self.buttons.push(Box::new(button));
    }
}