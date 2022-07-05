use raylib::prelude::*;
use crate::{Button, CanvasData, main};

pub struct NewButton {
    pub start_x: i32,
    pub start_y: i32
}

impl Button for NewButton {
    fn get_size(&self) -> (i32, i32) {
        (60, 20)
    }

    fn get_start(&self) -> (i32, i32) {
        (self.start_x, self.start_y)
    }

    fn run(&self, d: &mut RaylibDrawHandle, canvas_data: &mut CanvasData, mut canvas: &mut RenderTexture2D) {
        canvas_data.size = Vector2::new(5., 5.);
    }

    fn get_text(&self) -> String {
        String::from("New")
    }
}

pub fn set_white(size_x: i32, size_y: i32) -> Vec<u8> {
    let mut result: Vec<u8> = vec!();

    let mut i = 0;
    while i < size_x {
        let mut j = 0;
        while j < size_y {
            result.push(255);
            result.push(255);
            result.push(255);
            result.push(255);

            j += 1;
        }
        i += 1;
    }

    result
}