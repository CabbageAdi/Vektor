use raylib::prelude::*;

pub fn select_color(mouse_position: Vector2, size: i32, start_x: i32, start_y: i32, selected: Vector2) -> Vector2 {
    let end_x = start_x + size;
    let end_y = start_y + size;
    return if mouse_position.x < end_x as f32 && mouse_position.x > start_x as f32 && mouse_position.y > start_y as f32 && mouse_position.y < end_y as f32 {
        Vector2::new(mouse_position.x - start_x as f32, mouse_position.y - start_y as f32)
    } else {
        selected
    }
}

pub fn select_hsv(mouse_position: Vector2, size: i32, start_x: i32, start_y: i32, selected: f32) -> f32 {
    let start_x = start_x * 2 + size;
    let end_x = start_x + 35;
    let end_y = start_y + size;
    return if mouse_position.x < end_x as f32 && mouse_position.x > start_x as f32 && mouse_position.y > start_y as f32 && mouse_position.y < end_y as f32 {
        mouse_position.y - start_y as f32
    } else {
        selected
    }
}

pub fn draw_picker(d: &mut RaylibDrawHandle, size: i32, start_x: i32, start_y: i32, selected: Vector2, range_selected: f32, color: Color) {
    //square selector
    d.draw_circle_lines(selected.x as i32 + start_x, selected.y as i32 + start_y, 5., Color::BLACK);
    d.draw_circle_lines(selected.x as i32 + start_x, selected.y as i32 + start_y, 4., Color::BLACK);

    //strip selector
    d.draw_rectangle(start_x * 2 + size - 5, start_y + range_selected as i32, 35, 2, Color::BLACK);

    //rgb
    d.draw_text(&*format!("R: {} G: {} B: {}", color.r, color.g, color.b), start_x + 5, start_y * 2 + size - 15, 10, Color::WHITE)
}

pub fn set_picker_texture(color: Color, size: i32, texture: &mut RenderTexture2D) {
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

pub fn get_gradient_color(x: i32, y: i32, main_color: Color, size: i32) -> Color {
    let first = lerp_color(main_color, Color::WHITE, x as f32 / size as f32);
    let final_color = lerp_color(first, Color::BLACK, y as f32 / size as f32);
    return final_color;
}

pub fn set_range_texture(size: i32, texture: &mut RenderTexture2D) {
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

pub fn get_hsv(factor: f32) -> Color {
    Color::color_from_hsv(lerp(0., 360., factor), 1., 1.)
}

pub fn lerp_color(first: Color, second: Color, factor: f32) -> Color {
    Color::new(
        lerp(first.r as f32, second.r as f32, factor) as u8,
        lerp(first.g as f32, second.g as f32, factor) as u8,
        lerp(first.b as f32, second.b as f32, factor) as u8, 255)
}