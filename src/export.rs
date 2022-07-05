use raylib::prelude::*;
use rfd::FileDialog;
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

    fn run(&self, d: &mut RaylibDrawHandle, canvas_data: &mut CanvasData, canvas: &mut RenderTexture2D) {
        let files = FileDialog::new()
            .add_filter("png", &["png"])
            .set_file_name("Piksel.png")
            .set_directory(module_path!())
            .save_file()
            .unwrap();

        canvas.get_texture_data().unwrap().export_image(files.to_str().unwrap());
    }

    fn get_text(&self) -> String {
        String::from("Export")
    }
}
