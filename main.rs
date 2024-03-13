use macroquad::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CellState {
    Full,
    Empty,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "sand".to_owned(),
        window_width: 1000,
        window_height: 1000,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;
    let brush_size: i32 = 10;

    let mut cells = vec![CellState::Empty; w * h];
    let mut buffer = vec![CellState::Empty; w * h];

    let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);

    for cell in cells.iter_mut() {
        if rand::gen_range(0, 30) == 0 {
            *cell = CellState::Full;
        }
    }
    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(WHITE);

        let w = image.width();
        let h = image.height();

        for y in 0..h as i32 {
            for x in 0..w as i32 {
                let current_cell = cells[y as usize * w + x as usize];
                if current_cell == CellState::Empty {
                    continue;
                }
                if y + 1 >= h as i32 {
                    continue;
                }
                let bottom_cell = cells[(y + 1) as usize * w + x as usize];
                if bottom_cell == CellState::Empty {
                    buffer[y as usize * w + x as usize] = CellState::Empty;
                    buffer[(y + 1) as usize * w + x as usize] = CellState::Full;
                }
            }
        }

        let mouse_x: f32;
        let mouse_y: f32;
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
                    {
                        continue;
                    } else {
                        buffer[(mouse_y + (j as f32)) as usize * w
                            + (mouse_x + (i as f32)) as usize] = CellState::Full;
                    }
                }
            }
        }
        // if is_mouse_button_down(MouseButton::Right) {}

        for i in 0..buffer.len() {
            cells[i] = buffer[i];

            image.set_pixel(
                (i % w) as u32,
                (i / w) as u32,
                match buffer[i as usize] {
                    CellState::Full => GRAY,
                    CellState::Empty => WHITE,
                },
            );
        }

        texture.update(&image);

        draw_texture(&texture, 0., 0., WHITE);

        next_frame().await
    }
}

// #[derive(Copy, Clone, Debug, PartialEq, Eq)]
// enum CellState {
//     Alive,
//     Dead,
// }

// #[macroquad::main("Life")]
// async fn main() {
//     request_new_screen_size(500.0,500.0);
//     let w = screen_width() as usize;
//     let h = screen_height() as usize;

//     let mut cells = vec![CellState::Dead; w * h];
//     let mut buffer = vec![CellState::Dead; w * h];

//     let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);

//     for cell in cells.iter_mut() {
//         if rand::gen_range(0, 5) == 0 {
//             *cell = CellState::Alive;
//         }
//     }
//     let texture = Texture2D::from_image(&image);

//     loop {
//         clear_background(WHITE);

//         let w = image.width();
//         let h = image.height();

//         for y in 0..h as i32 {
//             for x in 0..w as i32 {
//                 let mut neighbors_count = 0;

//                 for j in -1i32..=1 {
//                     for i in -1i32..=1 {
//                         // out of bounds
//                         if y + j < 0 || y + j >= h as i32 || x + i < 0 || x + i >= w as i32 {
//                             continue;
//                         }
//                         // cell itself
//                         if i == 0 && j == 0 {
//                             continue;
//                         }

//                         let neighbor = cells[(y + j) as usize * w + (x + i) as usize];
//                         if neighbor == CellState::Alive {
//                             neighbors_count += 1;
//                         }
//                     }
//                 }

//                 let current_cell = cells[y as usize * w + x as usize];
//                 buffer[y as usize * w + x as usize] = match (current_cell, neighbors_count) {
//                     // Rule 1: Any live cell with fewer than two live neighbours
//                     // dies, as if caused by underpopulation.
//                     (CellState::Alive, x) if x < 2 => CellState::Dead,
//                     // Rule 2: Any live cell with two or three live neighbours
//                     // lives on to the next generation.
//                     (CellState::Alive, 2) | (CellState::Alive, 3) => CellState::Alive,
//                     // Rule 3: Any live cell with more than three live
//                     // neighbours dies, as if by overpopulation.
//                     (CellState::Alive, x) if x > 3 => CellState::Dead,
//                     // Rule 4: Any dead cell with exactly three live neighbours
//                     // becomes a live cell, as if by reproduction.
//                     (CellState::Dead, 3) => CellState::Alive,
//                     // All other cells remain in the same state.
//                     (otherwise, _) => otherwise,
//                 };
//             }
//         }

//         for i in 0..buffer.len() {
//             cells[i] = buffer[i];

//             image.set_pixel(
//                 (i % w) as u32,
//                 (i / w) as u32,
//                 match buffer[i as usize] {
//                     CellState::Alive => BLACK,
//                     CellState::Dead => WHITE,
//                 },
//             );
//         }

//         texture.update(&image);

//         draw_texture(&texture, 0., 0., WHITE);

//         next_frame().await
//     }
// }
