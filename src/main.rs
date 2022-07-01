mod picker;

use raylib::prelude::*;

fn main() {
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
        selected_color: Color::RED
    };

    //color picker
    let picker_start_x: i32 = 25;
    let picker_start_y: i32 = 25;
    let picker_size: i32 = 100;
    let mut picker_selected_position: Vector2 = Vector2::new(0., 0.);
    let mut range_selected_position: f32 = 0.;

    let mut started = false;

    let mut canvas: RenderTexture2D = rl.load_render_texture(&thread, canvas_data.size.x as u32, canvas_data.size.y as u32).expect("error initializing canvas");
    let mut picker_texture: RenderTexture2D = rl.load_render_texture(&thread, picker_size as u32, picker_size as u32).expect("error");
    let mut range_texture: RenderTexture2D = rl.load_render_texture(&thread, 25, picker_size as u32).expect("");

    let mut mouse_last_position: Vector2 = Vector2::zero();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::DARKGRAY);

        let mouse_position = d.get_mouse_position();
        let height = d.get_screen_height();
        let width = d.get_screen_width();

        //draw stuff
        let mut x_end = canvas_data.start_x + (canvas_data.size.x * canvas_data.scale);
        let mut y_end = canvas_data.start_y + (canvas_data.size.y * canvas_data.scale);

        if x_end > width as f32 { x_end = width as f32; }
        if y_end > height as f32 { y_end = height as f32; }

        d.draw_texture_ex(&canvas, Vector2::new(canvas_data.start_x, canvas_data.start_y), 0., canvas_data.scale, Color::WHITE);
        d.draw_rectangle(0, 0, picker_start_x * 3 + 25 + picker_size, height, Color::GRAY);
        d.draw_texture(&picker_texture, picker_start_x, picker_start_y, Color::WHITE);
        d.draw_texture(&range_texture, picker_start_x * 2 + picker_size, picker_start_y, Color::WHITE);
        picker::draw_picker(&mut d, picker_size, picker_start_x, picker_start_y, picker_selected_position, range_selected_position, canvas_data.selected_color);

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
        if !started {
            picker::set_picker_texture(canvas_data.selected_color_range, picker_size, &mut picker_texture);
            picker::set_range_texture(picker_size, &mut range_texture);
        }

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

        //draw on canvas
        let mut texture_stream = d.begin_texture_mode(&thread, &mut canvas);

        if !started {
            texture_stream.clear_background(Color::WHITE);
            started = true;
        }

        //draw
        if texture_stream.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            if mouse_position.x < x_end && mouse_position.x > canvas_data.start_x && mouse_position.y > canvas_data.start_y && mouse_position.y < y_end {
                let pixel_pos_x = ((mouse_position.x - canvas_data.start_x) / canvas_data.scale).floor();
                let pixel_pos_y = ((mouse_position.y - canvas_data.start_y) / canvas_data.scale).floor();
                texture_stream.draw_pixel(pixel_pos_x as i32,  canvas_data.size.y as i32 - pixel_pos_y as i32 - 1, canvas_data.selected_color);
            }
        }

        mouse_last_position = mouse_position;
    }
}

struct CanvasData {
    scale: f32,
    size: Vector2,
    start_x: f32,
    start_y: f32,
    scale_sensitivity: f32,
    selected_color_range: Color,
    selected_color: Color
}
