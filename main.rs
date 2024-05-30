use std::{fmt::format, i32};

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
        window_title: "sand".to_owned(),
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
    let mut brush_size: i32 = 20;
    let mut color_index = 1;

    let mut cells = vec![Cell(CellState::Empty, BACKGROUND); w * h];
    let mut buffer = vec![Cell(CellState::Empty, BACKGROUND); w * h];
    let mut image = Image::gen_image_color(w as u16, h as u16, BACKGROUND);

    let color_palette = [
        BEIGE, BLACK, BLANK, BLUE, BROWN, DARKBLUE, DARKBROWN, DARKGRAY, DARKGREEN, DARKPURPLE,
        GOLD, GRAY, GREEN, LIGHTGRAY, LIME, MAGENTA, MAROON, ORANGE, PINK, PURPLE, RED, SKYBLUE,
        VIOLET, YELLOW,
    ];
    let color_palette_names = [
        "BEIGE",
        "BLACK",
        "BLANK",
        "BLUE",
        "BROWN",
        "DARKBLUE",
        "DARKBROWN",
        "DARKGRAY",
        "DARKGREEN",
        "DARKPURPLE",
        "GOLD",
        "GRAY",
        "GREEN",
        "LIGHTGRAY",
        "LIME",
        "MAGENTA",
        "MAROON",
        "ORANGE",
        "PINK",
        "PURPLE",
        "RED",
        "SKYBLUE",
        "VIOLET",
        "YELLOW",
    ];

    // for cell in cells.iter_mut() {
    // if rand::gen_range(0, 30) == 0 {
    // *cell = Cell(CellState::Full, BLUE);
    // }
    // }
    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(WHITE);

        let w = image.width();
        let h = image.height();

        for y in 0..h as i32 {
            for x in 0..w as i32 {
                let current_cell = cells[y as usize * w + x as usize];
                if current_cell.0 == CellState::Empty {
                    continue;
                }
                if y + 2 >= h as i32 {
                    continue;
                }
                let bottom_cell = cells[(y + 1) as usize * w + x as usize];
                if bottom_cell.0 == CellState::Empty {
                    buffer[y as usize * w + x as usize].0 = CellState::Empty;
                    buffer[(y + 1) as usize * w + x as usize].0 = CellState::Full;
                    buffer[(y + 1) as usize * w + x as usize].1 =
                        buffer[y as usize * w + x as usize].1;
                    buffer[y as usize * w + x as usize].1 = BACKGROUND;
                } else if x - 1 >= 0
                    && cells[(y + 1) as usize * w + (x - 1) as usize].0 == CellState::Empty
                {
                    buffer[y as usize * w + x as usize].0 = CellState::Empty;
                    buffer[(y + 1) as usize * w + (x - 1) as usize].0 = CellState::Full;
                    buffer[(y + 1) as usize * w + (x - 1) as usize].1 =
                        buffer[y as usize * w + x as usize].1;
                    buffer[y as usize * w + x as usize].1 = BACKGROUND;
                } else if x + 1 < w as i32
                    && cells[(y + 1) as usize * w + (x + 1) as usize].0 == CellState::Empty
                {
                    buffer[y as usize * w + x as usize].0 = CellState::Empty;
                    buffer[(y + 1) as usize * w + (x + 1) as usize].0 = CellState::Full;
                    buffer[(y + 1) as usize * w + (x + 1) as usize].1 =
                        buffer[y as usize * w + x as usize].1;
                    buffer[y as usize * w + x as usize].1 = BACKGROUND;
                }
            }
        }

        let mut mouse_x: f32;
        let mut mouse_y: f32;

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
        if is_key_down(KeyCode::Up) && brush_size < h as i32 / 3 {
            brush_size += 1;
        }
        if is_key_down(KeyCode::Down) && brush_size > 0 {
            brush_size -= 1;
        }

        if is_mouse_button_down(MouseButton::Left) {
            (mouse_x, mouse_y) = mouse_position();
            for j in -brush_size..=brush_size {
                for i in -brush_size..=brush_size {
                    // out of bounds
                    if ((i * i + j * j) as f32).sqrt() >= brush_size as f32 {
                        continue;
                    }
                    if mouse_y + (j as f32) < 0.0
                        || mouse_y + (j as f32) >= h as f32
                        || mouse_x + (i as f32) < 0.0
                        || mouse_x + (i as f32) >= w as f32
                        || rand::gen_range(0, 2) == 0
                    {
                        continue;
                    } else {
                        buffer[(mouse_y + (j as f32)) as usize * w
                            + (mouse_x + (i as f32)) as usize]
                            .0 = CellState::Full;
                        buffer[(mouse_y + (j as f32)) as usize * w
                            + (mouse_x + (i as f32)) as usize]
                            .1 = color_palette[color_index];
                    }
                }
            }
        }
        if is_mouse_button_down(MouseButton::Right) {
            (mouse_x, mouse_y) = mouse_position();
            for j in -brush_size..=brush_size {
                for i in -brush_size..=brush_size {
                    // out of bounds
                    if ((i * i + j * j) as f32).sqrt() >= brush_size as f32 {
                        continue;
                    }
                    if mouse_y + (j as f32) < 0.0
                        || mouse_y + (j as f32) >= h as f32
                        || mouse_x + (i as f32) < 0.0
                        || mouse_x + (i as f32) >= w as f32
                    {
                        continue;
                    } else {
                        buffer[(mouse_y + (j as f32)) as usize * w
                            + (mouse_x + (i as f32)) as usize]
                            .0 = CellState::Empty;
                        buffer[(mouse_y + (j as f32)) as usize * w
                            + (mouse_x + (i as f32)) as usize]
                            .1 = BACKGROUND;
                    }
                }
            }
        }

        for i in 0..buffer.len() {
            cells[i] = buffer[i];
            image.set_pixel((i % w) as u32, (i / w) as u32, buffer[i as usize].1);
        }

        texture.update(&image);
        let bs = format!("{}", brush_size);
        draw_texture(&texture, 0., 0., WHITE);
        draw_text(bs.as_str(), 20.0, 20.0, 30.0, DARKGRAY);
        draw_rectangle(
            w as f32 - 30.0,
            10.0,
            20.0,
            20.0,
            color_palette[color_index],
        );
        next_frame().await
    }
}
//pink == Color::new(1.0, 0.4, 0.7, 1.0);
