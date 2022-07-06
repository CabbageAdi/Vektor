use raylib::prelude::*;
use rfd::FileDialog;
use crate::{ButtonState, CanvasData, UI};
use crate::ui::Button;

pub struct ExportButton {
    pub start_x: i32,
    pub start_y: i32
}

pub struct ExportRun;

impl ButtonState for ExportButton {
    fn get_size(&mut self) -> (i32, i32) {
        (60, 20)
    }

    fn get_start(&mut self) -> (i32, i32) {
        (self.start_x, self.start_y)
    }

    fn get_text(&mut self) -> String {
        String::from("Export")
    }

    fn enabled(&mut self) -> bool {
        true
    }

    fn set_enabled(&mut self, enable: bool) {
        if !enable {
            panic!("cannot disable export button");
        }
    }
}

impl Button for ExportRun {
    fn run(&self, d: &mut RaylibDrawHandle, canvas_data: &mut CanvasData, canvas: &mut RenderTexture2D, ui: &mut UI) {
        let files = FileDialog::new()
            .add_filter("png", &["png"])
            .set_file_name("Piksel.png")
            .set_directory(module_path!())
            .save_file()
            .unwrap();

        canvas.get_texture_data().unwrap().export_image(files.to_str().unwrap());
    }
}
