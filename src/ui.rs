use raylib::prelude::RenderTexture2D;
use crate::CanvasData;

pub trait Button {
    //todo: make responsive somehow
    fn get_size(&self) -> (i32, i32);
    //temporary hopefully
    fn get_start(&self) -> (i32, i32);
    fn run(&self, canvas_data: &CanvasData, canvas: &RenderTexture2D);
    fn get_text(&self) -> String;
}
