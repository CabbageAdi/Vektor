use raylib::prelude::*;
use crate::{Button, ButtonState, CanvasData, main, UI};

pub struct NewButton {
    pub start_x: i32,
    pub start_y: i32
}

pub struct NewRun;

impl ButtonState for NewButton {
    fn get_size(&mut self) -> (i32, i32) {
        (60, 20)
    }

    fn get_start(&mut self) -> (i32, i32) {
        (self.start_x, self.start_y)
    }

    fn get_text(&mut self) -> String {
        String::from("New")
    }

    fn enabled(&mut self) -> bool {
        true
    }

    fn set_enabled(&mut self, enable: bool) {
        if !enable {
            panic!("cannot disable new button");
        }
    }

    fn get_behavior(&self) -> Box<dyn Button> {
        Box::new(NewRun {})
    }
}

impl Button for NewRun {
    fn run(&self, d: &mut RaylibDrawHandle, canvas_data: &mut CanvasData, mut canvas: &mut RenderTexture2D, ui: &mut UI) {
        canvas_data.size_x = 5;
        canvas_data.size_y = 5;
        canvas_data.started = false;
    }
}

pub fn set_white(size_x: i32, size_y: i32) -> Vec<u8> {
    vec![255; (size_x * size_y * 4) as usize]
}
