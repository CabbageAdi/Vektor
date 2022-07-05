use raylib::prelude::*;
use crate::CanvasData;

pub trait Button {
    //todo: make responsive somehow
    fn get_size(&self) -> (i32, i32);
    //temporary hopefully
    fn get_start(&self) -> (i32, i32);
    fn run(&self, d: &mut RaylibDrawHandle, canvas_data: &mut CanvasData, canvas: &mut RenderTexture2D);
    fn get_text(&self) -> String;
}
