use raylib::prelude::*;

fn main() {
    let (mut rl, mut thread) = raylib::init()
        .size(1280, 720)
        .title("Piksel | By Cabbage")
        .resizable()
        .build();

    let mut scale: f32 = 15.;
    let mut canvas_size = Vector2::new(20.0, 20.0);
    let mut start_y: f32 = 100.;
    let mut start_x: f32 = 300.;
    let mut scale_sensitivity = 0.4;

    //color picker
    let mut selected_color_range: Color = Color::RED;
    let picker_start_x: i32 = 25;
    let picker_start_y: i32 = 25;
    let picker_size: i32 = 100;
    let mut picker_selected_position: Vector2 = Vector2::new(0., 0.);
    let mut range_selected_position: f32 = 0.;
    let mut selected_color: Color = Color::RED;

    let mut started = false;

    let mut canvas: RenderTexture2D = rl.load_render_texture(&thread, canvas_size.x as u32, canvas_size.y as u32).expect("error initializing canvas");
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
        let mut x_end = start_x + (canvas_size.x * scale);
        let mut y_end = start_y + (canvas_size.y * scale);

        if x_end > width as f32 { x_end = width as f32; }
        if y_end > height as f32 { y_end = height as f32; }

        d.draw_texture_ex(&canvas, Vector2::new(start_x, start_y), 0., scale, Color::WHITE);
        d.draw_rectangle(0, 0, picker_start_x * 3 + 25 + picker_size, height, Color::GRAY);
        d.draw_texture(&picker_texture, picker_start_x, picker_start_y, Color::WHITE);
        d.draw_texture(&range_texture, picker_start_x * 2 + picker_size, picker_start_y, Color::WHITE);
        draw_picker(&mut d, picker_size, picker_start_x, picker_start_y, picker_selected_position, range_selected_position);

        //zoom
        if d.get_mouse_wheel_move() != 0. {
            if d.get_mouse_wheel_move() < 0. {
                scale -= scale_sensitivity;
            }
            else {
                scale += scale_sensitivity;
            }
        }

        //pan
        if d.is_mouse_button_down(MouseButton::MOUSE_MIDDLE_BUTTON) {
            if mouse_last_position != mouse_position {
                let change = mouse_position - mouse_last_position;
                start_x += change.x;
                start_y += change.y;
            }
        }

        //color picker
        if !started {
            set_picker_texture(selected_color_range, picker_size, &mut picker_texture);
            set_range_texture(picker_size, &mut range_texture);
        }

        let prev_range = range_selected_position;
        if d.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            //picker and range
            picker_selected_position = select_color(mouse_position, picker_size, picker_start_x, picker_start_y, picker_selected_position);
            selected_color = get_gradient_color(picker_selected_position.x as i32, picker_selected_position.y as i32, selected_color_range, picker_size);

            range_selected_position = select_hsv(mouse_position, picker_size, picker_start_x, picker_start_y, range_selected_position);
            if prev_range != range_selected_position {
                selected_color_range = get_hsv(range_selected_position / picker_size as f32);
                set_picker_texture(selected_color_range, picker_size, &mut picker_texture);
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
            if mouse_position.x < x_end && mouse_position.x > start_x && mouse_position.y > start_y && mouse_position.y < y_end {
                let pixel_pos_x = ((mouse_position.x - start_x) / scale).floor();
                let pixel_pos_y = ((mouse_position.y - start_y) / scale).floor();
                texture_stream.draw_pixel(pixel_pos_x as i32,  canvas_size.y as i32 - pixel_pos_y as i32 - 1, selected_color);
            }
        }

        mouse_last_position = mouse_position;
    }
}

fn select_color(mouse_position: Vector2, size: i32, start_x: i32, start_y: i32, selected: Vector2) -> Vector2 {
    let end_x = start_x + size;
    let end_y = start_y + size;
    return if mouse_position.x < end_x as f32 && mouse_position.x > start_x as f32 && mouse_position.y > start_y as f32 && mouse_position.y < end_y as f32 {
        Vector2::new(mouse_position.x - start_x as f32, mouse_position.y - start_y as f32)
    } else {
        selected
    }
}

fn select_hsv(mouse_position: Vector2, size: i32, start_x: i32, start_y: i32, selected: f32) -> f32 {
    let start_x = start_x * 2 + size;
    let end_x = start_x + 35;
    let end_y = start_y + size;
    return if mouse_position.x < end_x as f32 && mouse_position.x > start_x as f32 && mouse_position.y > start_y as f32 && mouse_position.y < end_y as f32 {
        mouse_position.y - start_y as f32
    } else {
        selected
    }
}

fn draw_picker(d: &mut RaylibDrawHandle, size: i32, start_x: i32, start_y: i32, selected: Vector2, range_selected: f32) {
    //square selector
    d.draw_circle_lines(selected.x as i32 + start_x, selected.y as i32 + start_y, 5., Color::BLACK);
    d.draw_circle_lines(selected.x as i32 + start_x, selected.y as i32 + start_y, 4., Color::BLACK);

    //strip selector
    d.draw_rectangle(start_x * 2 + size - 5, start_y + range_selected as i32, 35, 3, Color::BLACK);
}

fn set_picker_texture(color: Color, size: i32, texture: &mut RenderTexture2D) {
    let mut result: Vec<u8> = vec![];

    let mut i = 0;
    while i < size {
        let mut j = 0;
        while j < size {
            let pixel_color = get_gradient_color(j, i, color, size);
            result.push(pixel_color.r);
            result.push(pixel_color.g);
            result.push(pixel_color.b);
            result.push(pixel_color.a);
            j += 1;
        }
        i += 1;
    }
    texture.update_texture(result.as_ref());
}

fn get_gradient_color(x: i32, y: i32, main_color: Color, size: i32) -> Color {
    let first = lerp_color(main_color, Color::WHITE, x as f32 / size as f32);
    let final_color = lerp_color(first, Color::BLACK, y as f32 / size as f32);
    return final_color;
}

fn set_range_texture(size: i32, texture: &mut RenderTexture2D) {
    let mut result: Vec<u8> = vec![];

    let mut j = 0;
    while j < size {
        let mut i = 0;
        while i < 25 {
            let color = get_hsv(j as f32 / size as f32);
            result.push(color.r);
            result.push(color.g);
            result.push(color.b);
            result.push(color.a);
            i += 1;
        }
        j += 1;
    }

    texture.update_texture(result.as_ref());
}

fn get_hsv(factor: f32) -> Color {
    Color::color_from_hsv(lerp(0., 360., factor), 1., 1.)
}

fn lerp_color(first: Color, second: Color, factor: f32) -> Color {
    Color::new(
        lerp(first.r as f32, second.r as f32, factor) as u8,
        lerp(first.g as f32, second.g as f32, factor) as u8,
        lerp(first.b as f32, second.b as f32, factor) as u8, 255)
}
