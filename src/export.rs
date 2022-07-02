use raylib::prelude::RenderTexture2D;
use rfd::AsyncFileDialog;
use crate::CanvasData;
use crate::ui::Button;

pub struct ExportButton {
    pub start_x: i32,
    pub start_y: i32
}

impl Button for ExportButton {
    fn get_size(&self) -> (i32, i32) {
        (60, 20)
    }

    fn get_start(&self) -> (i32, i32) {
        (self.start_x, self.start_y)
    }

    fn run(&self, canvas_data: &CanvasData, canvas: &RenderTexture2D) {
        println!("clicked");
    }

    fn get_text(&self) -> String {
        String::from("Export")
    }
}
