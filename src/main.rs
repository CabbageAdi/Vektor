extern crate core;

mod picker;
mod ui;
mod export;
mod new;

use raylib::prelude::*;
use crate::export::*;
use crate::new::*;
use crate::ui::*;

pub fn main() {
    let (mut rl, mut thread) = raylib::init()
        .size(1280, 720)
        .title("Piksel | By Cabbage")
        .resizable()
        .build();

    let mut canvas_data: CanvasData = CanvasData {
        scale: 15.,
        size: Vector2::new(20., 20.),
        start_x: 300.,
        start_y: 100.,
        scale_sensitivity: 0.4,
        selected_color_range: Color::RED,
        selected_color: Color::RED,
        started: false
    };

    //color picker
    let picker_start_x: i32 = 25;
    let picker_start_y: i32 = 25;
    let picker_size: i32 = 100;
    let mut picker_selected_position: Vector2 = Vector2::new(0., 0.);
    let mut range_selected_position: f32 = 0.;

    let mut ui = UI {
        //todo: remove dependance on picker variables
        buttons: vec!(
            Box::new(ExportButton{
                start_x: picker_start_x,
                start_y: picker_size + picker_start_y * 2
            }),
            Box::new(NewButton {
                start_x: picker_start_x * 2 + 60,
                start_y: picker_size + picker_start_y * 2
            })
        ),
        panels: vec!()
    };

    let mut buttons: Vec<Box<dyn Button>> = vec!(Box::new(ExportRun{}), Box::new(NewRun{}));

    let mut canvas: RenderTexture2D = rl.load_render_texture(&thread, canvas_data.size.x as u32, canvas_data.size.y as u32).expect("error initializing canvas");
    let mut picker_texture: RenderTexture2D = rl.load_render_texture(&thread, picker_size as u32, picker_size as u32).expect("error");
    let mut range_texture: RenderTexture2D = rl.load_render_texture(&thread, 25, picker_size as u32).expect("");

    picker::set_picker_texture(canvas_data.selected_color_range, picker_size, &mut picker_texture);
    picker::set_range_texture(picker_size, &mut range_texture);

    let mut mouse_last_position: Vector2 = Vector2::zero();

    while !rl.window_should_close() {
        if !canvas_data.started {
            canvas = rl.load_render_texture(&thread, canvas_data.size.x as u32, canvas_data.size.y as u32).expect("error initializing canvas");
            canvas.update_texture(&*new::set_white(canvas_data.size.x as i32, canvas_data.size.y as i32));
            canvas_data.started = true;
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::DARKGRAY);

        let mouse_position = d.get_mouse_position();
        let height = d.get_screen_height();
        let width = d.get_screen_width();

        //draw stuff
        let mut end_x = canvas_data.start_x + (canvas_data.size.x * canvas_data.scale);
        let mut end_y = canvas_data.start_y + (canvas_data.size.y * canvas_data.scale);

        if end_x > width as f32 { end_x = width as f32; }
        if end_y > height as f32 { end_y = height as f32; }

        //canvas draw
        d.draw_texture_ex(&canvas, Vector2::new(canvas_data.start_x, canvas_data.start_y), 0., canvas_data.scale, Color::WHITE);
        //side panel draw
        d.draw_rectangle(0, 0, picker_start_x * 3 + 25 + picker_size, height, Color::GRAY);
        //picker draw
        d.draw_texture(&picker_texture, picker_start_x, picker_start_y, Color::WHITE);
        d.draw_texture(&range_texture, picker_start_x * 2 + picker_size, picker_start_y, Color::WHITE);
        picker::draw_picker(&mut d, picker_size, picker_start_x, picker_start_y, picker_selected_position, range_selected_position, canvas_data.selected_color);

        //buttons
        // let button_start_y = picker_size + picker_start_y * 2;
        // let button_start_x = picker_start_x;
        for button in ui.buttons.iter_mut() {
            if button.enabled() {
                let start = button.get_start();
                draw_button(&mut d, button, start.0, start.1);
            }
        }

        //zoom
        if d.get_mouse_wheel_move() != 0. {
            if d.get_mouse_wheel_move() < 0. {
                canvas_data.scale -= canvas_data.scale_sensitivity;
            }
            else {
                canvas_data.scale += canvas_data.scale_sensitivity;
            }
        }

        //pan
        if d.is_mouse_button_down(MouseButton::MOUSE_MIDDLE_BUTTON) {
            if mouse_last_position != mouse_position {
                let change = mouse_position - mouse_last_position;
                canvas_data.start_x += change.x;
                canvas_data.start_y += change.y;
            }
        }

        //color picker
        let prev_range = range_selected_position;
        if d.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            //picker and range
            picker_selected_position = picker::select_color(mouse_position, picker_size, picker_start_x, picker_start_y, picker_selected_position);
            canvas_data.selected_color = picker::get_gradient_color(picker_selected_position.x as i32, picker_selected_position.y as i32, canvas_data.selected_color_range, picker_size);

            range_selected_position = picker::select_hsv(mouse_position, picker_size, picker_start_x, picker_start_y, range_selected_position);
            if prev_range != range_selected_position {
                canvas_data.selected_color_range = picker::get_hsv(range_selected_position / picker_size as f32);
                picker::set_picker_texture(canvas_data.selected_color_range, picker_size, &mut picker_texture);
            }
        }

        if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            let mut i = 0;
            while i < buttons.len() {
                let button_state = &mut ui.buttons[i];
                let start = &button_state.get_start();
                let size = &button_state.get_size();
                if mouse_bounds(mouse_position, start.0, start.1, start.0 + size.0, start.1 + size.1) {
                    buttons[i].run(&mut d, &mut canvas_data, &mut canvas, &mut ui);
                };
                i += 1;
            }
        }

        //draw on canvas
        let mut texture_stream = d.begin_texture_mode(&thread, &mut canvas);

        if texture_stream.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            if mouse_bounds(mouse_position, canvas_data.start_x as i32, canvas_data.start_y as i32, end_x as i32, end_y as i32) {
                let pixel_pos_x = ((mouse_position.x - canvas_data.start_x) / canvas_data.scale).floor();
                let pixel_pos_y = ((mouse_position.y - canvas_data.start_y) / canvas_data.scale).floor();
                texture_stream.draw_pixel(pixel_pos_x as i32,  canvas_data.size.y as i32 - pixel_pos_y as i32 - 1, canvas_data.selected_color);
            }
        }

        mouse_last_position = mouse_position;
    }
}

fn draw_button(d: &mut RaylibDrawHandle, button: &mut Box<dyn ButtonState>, start_x: i32, start_y: i32) {
    let size = button.get_size();
    d.draw_rectangle(start_x, start_y, size.0, size.1, Color::DARKGRAY);

    let text = button.get_text();
    let font_size = size.1 - 10;
    let length = measure_text(&*text, font_size);
    d.draw_text(&*text, start_x + (size.0 - length) / 2, start_y + 5, font_size, Color::WHITE);
}

fn mouse_bounds(mouse_position: Vector2, start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> bool {
    mouse_position.x < end_x as f32 && mouse_position.x > start_x as f32 && mouse_position.y > start_y as f32 && mouse_position.y < end_y as f32
}

#[derive(Clone, Copy)]
pub struct CanvasData {
    scale: f32,
    size: Vector2,
    start_x: f32,
    start_y: f32,
    scale_sensitivity: f32,
    selected_color_range: Color,
    selected_color: Color,
    started: bool
}
