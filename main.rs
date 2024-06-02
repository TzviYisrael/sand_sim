//basic sand simulator, made by Tzvi Yisrael
use macroquad::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CellState {
    Full,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Cell(CellState, Color);

fn window_conf() -> Conf {
    Conf {
        window_title: "sand_sim".to_owned(),
        window_width: 1500,
        window_height: 1500,
        // fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    const BACKGROUND: Color = WHITE;
    let w = screen_width() as usize;
    let h = screen_height() as usize;
    let mut brush_size: f32 = 40.0;
    let mut color_index: usize = 10;
    let mut optimisation_line: usize = h;
    let mut help_message_time: i32 = 200;

    let mut cells = vec![Cell(CellState::Empty, BACKGROUND); w * h];
    let mut buffer = vec![Cell(CellState::Empty, BACKGROUND); w * h];
    let mut image = Image::gen_image_color(w as u16, h as u16, BACKGROUND);

    let color_palette = [
        BEIGE, BLACK, BLANK, BLUE, BROWN, DARKBLUE, DARKBROWN, DARKGRAY, DARKGREEN, DARKPURPLE,
        GOLD, GRAY, GREEN, LIGHTGRAY, LIME, MAGENTA, MAROON, ORANGE, PINK, PURPLE, RED, SKYBLUE,
        VIOLET, YELLOW,
    ];

    let texture = Texture2D::from_image(&image);

    let mut mouse_x: f32;
    let mut mouse_y: f32;

    loop {
        clear_background(WHITE);

        let w = image.width();
        let h = image.height();

        (mouse_x, mouse_y) = mouse_position();

        //find the first line that have an empty cell
        let mut full_line = true;
        for y in (0..(optimisation_line - 1)).rev() {
            for x in 0..w {
                if cells[y * w + x].0 == CellState::Empty {
                    full_line = false;
                    break;
                }
            }
            if full_line {
                if optimisation_line > 0 {
                    optimisation_line -= 1;
                }
            } else {
                break;
            }
        }

        for y in 0..optimisation_line {
            for x in 0..w {
                let current_cell = cells[y * w + x];
                if current_cell.0 == CellState::Empty {
                    continue;
                }
                if y + 1 >= h {
                    continue;
                }
                let bottom_cell = cells[(y + 1) * w + x];
                if bottom_cell.0 == CellState::Empty {
                    buffer[y * w + x].0 = CellState::Empty;
                    buffer[(y + 1) * w + x].0 = CellState::Full;
                    buffer[(y + 1) * w + x].1 = buffer[y * w + x].1;
                    buffer[y * w + x].1 = BACKGROUND;
                } else if x >= 1 && cells[(y + 1) * w + (x - 1)].0 == CellState::Empty {
                    buffer[y * w + x].0 = CellState::Empty;
                    buffer[(y + 1) * w + (x - 1)].0 = CellState::Full;
                    buffer[(y + 1) * w + (x - 1)].1 = buffer[y * w + x].1;
                    buffer[y * w + x].1 = BACKGROUND;
                } else if x + 1 < w && cells[(y + 1) * w + (x + 1)].0 == CellState::Empty {
                    buffer[y * w + x].0 = CellState::Empty;
                    buffer[(y + 1) * w + (x + 1)].0 = CellState::Full;
                    buffer[(y + 1) * w + (x + 1)].1 = buffer[y * w + x].1;
                    buffer[y * w + x].1 = BACKGROUND;
                }
            }
        }
        //keyboard input
        if is_key_pressed(KeyCode::Right) {
            color_index += 1;
            if color_index == color_palette.len() - 1 {
                color_index = 0;
            }
        }
        if is_key_pressed(KeyCode::Left) {
            color_index -= 1;
            if color_index == 0 {
                color_index = color_palette.len() - 1;
            }
        }
        if is_key_down(KeyCode::Up) && brush_size < h as f32 / 3.0 {
            brush_size += 1.0;
        }
        if is_key_down(KeyCode::Down) && brush_size > 0.0 {
            brush_size -= 1.0;
        }

        let (_mouse_wheel_x, mouse_wheel_y) = mouse_wheel();
        if is_mouse_button_down(MouseButton::Middle) {
            if mouse_wheel_y < 0.0 {
                if color_index == 0 {
                    color_index = color_palette.len() - 1;
                } else {
                    color_index -= 1;
                }
            } else if mouse_wheel_y > 0.0 {
                if color_index >= color_palette.len() - 1 {
                    color_index = 0;
                } else {
                    color_index += 1;
                }
            }
        } else {
            brush_size += mouse_wheel_y * 5.0;
            if brush_size < 0.0 {
                brush_size = 0.0;
            } else if brush_size < h as f32 / 3.0 {
                brush_size = h as f32 / 3.0;
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            (mouse_x, mouse_y) = mouse_position();
            for _j in -brush_size as i32..=brush_size as i32 {
                let j = _j as f32;
                for _i in -brush_size as i32..=brush_size as i32 {
                    let i = _i as f32;
                    // out of bounds
                    if (i * i + j * j).sqrt() >= brush_size {
                        continue;
                    }
                    if mouse_y + j < 0.0
                        || mouse_y + j >= h as f32
                        || mouse_x + i < 0.0
                        || mouse_x + i >= w as f32
                        || rand::gen_range(0, 2) == 0
                    {
                        continue;
                    } else {
                        buffer[(mouse_y + j) as usize * w + (mouse_x + i) as usize].0 =
                            CellState::Full;
                        buffer[(mouse_y + j) as usize * w + (mouse_x + i) as usize].1 =
                            color_palette[color_index];
                    }
                }
            }
        }

        if is_mouse_button_down(MouseButton::Right) {
            for _j in -brush_size as i32..=brush_size as i32 {
                let j = _j as f32;
                for _i in -brush_size as i32..=brush_size as i32 {
                    let i = _i as f32;
                    // out of bounds
                    if (i * i + j * j).sqrt() >= brush_size {
                        continue;
                    }
                    if mouse_y + j < 0.0
                        || mouse_y + j >= h as f32
                        || mouse_x + i < 0.0
                        || mouse_x + i >= w as f32
                    {
                        continue;
                    } else {
                        buffer[(mouse_y + j) as usize * w + (mouse_x + i) as usize].0 =
                            CellState::Empty;
                        buffer[(mouse_y + j) as usize * w + (mouse_x + i) as usize].1 = BACKGROUND;
                    }
                }
            }
            //reset the optimisation line
            optimisation_line = h;
        }

        for i in 0..buffer.len() {
            cells[i] = buffer[i];
            image.set_pixel((i % w) as u32, (i / w) as u32, buffer[i as usize].1);
        }

        texture.update(&image);
        draw_texture(&texture, 0., 0., WHITE);

        //UI
        draw_circle(w as f32 - 30.0, 30.0, 15.0, color_palette[color_index]);
        draw_circle_lines(
            mouse_x,
            mouse_y,
            brush_size,
            2.0,
            color_palette[color_index],
        );
        if help_message_time > 0 {
            draw_text(
                "left mouse buttom to add sand",
                w as f32 / 2.0 - 200.0,
                120.0,
                30.0,
                DARKGRAY,
            );
            draw_text(
                "right mouse buttom to remove sand",
                w as f32 / 2.0 - 220.0,
                140.0,
                30.0,
                DARKGRAY,
            );
            draw_text(
                "roll mouse wheel to change brush size",
                w as f32 / 2.0 - 250.0,
                180.0,
                30.0,
                DARKGRAY,
            );
        }
        if help_message_time > -70 {
            draw_text(
                "press and roll mouse wheel to change color",
                w as f32 / 2.0 - 270.0,
                200.0,
                30.0,
                DARKGRAY,
            );

            help_message_time -= 1;
        }
        //dbg
        // draw_line(0.0,optimisation_line as f32,w as f32,optimisation_line as f32,2.0,RED,);

        next_frame().await
    }
}
