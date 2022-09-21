use macroquad::prelude::*;
use macroquad::rand;
use macroquad::rand::rand;

use std::time::{SystemTime, UNIX_EPOCH};



#[macroquad::main(window_conf)]
async fn main() {


    // General setting
    
    let mut paused = true; 

    let mut frame_count: u32 = 0;
    let mut board: Vec<bool>;
    let mut temp_board: Vec<bool>;
    let width = 400;
    let height = 100;

    let mut mouse_cell = Vec2::ZERO;


    // init board
    board = randomize_board(width, height);
    temp_board = vec![false; (width * height) as usize];

    loop {

        if !paused && frame_count % 4 == 0 {

            // Update board if not paused
            for (index, cell) in board.iter().enumerate() {
                let cell_x = (index % width) as i32;
                let cell_y = (index / width) as i32;
                let neighbour: [(i32, i32); 8] = [
                    (-1, -1),
                    (-1,0),
                    (-1,1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1)
                ];

                let mut count = 0;
                for n in neighbour.iter() {
                    if get_cell_value(cell_x + n.0, cell_y + n.1, width as i32, height as i32, &board) {
                        count += 1;
                    }
                }


                // rules of life 
                if count == 3 && !cell {
                    temp_board[index] = true;
                }
                else if *cell && (count == 3 || count == 2) {
                    temp_board[index] = true;
                }
                else {
                    temp_board[index] = false;
                }

            }

            // Copy temp board to active board
            for (index, cell) in temp_board.iter().enumerate() {
                board[index] = *cell;
            }
        }
        // If paused, draw or remove cells
        else {
            mouse_cell.x = (mouse_position().0 / 6.0).round();
            mouse_cell.y = (mouse_position().1 / 6.0).round();
            if is_mouse_button_pressed(MouseButton::Left) {
                let index = (mouse_cell.x + mouse_cell.y * width as f32) as usize;
                board[index] = !board[index];
            }

        }


        // Draw board
        for (index, cell) in board.iter().enumerate() {
            if *cell {
                let x = (index % width) as f32 * 6.0;
                let y = (index / width) as f32 * 6.0;
                draw_circle_lines(x, y, 2.0, 1.0, WHITE);
            }
        }


        // Check inputs
        if is_key_pressed(KeyCode::R) {
            board = randomize_board(width, height);
        } else if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        } else if is_key_pressed(KeyCode::C) {
            board = clear_board(width, height);

        }

        frame_count += 1; 
        next_frame().await
    }
}

/// Clear all the board
fn clear_board(width: usize, height: usize) -> Vec<bool> {
    vec![false; width * height]
}


/// Create a randomized board
fn randomize_board(width: usize, height: usize) -> Vec<bool> {
    let mut board = Vec::new();
    // Seed the random generator
    rand::srand(SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis() as u64);
    for _i in 0..(width * height) {
        let alea = rand() % 100;
        if alea < 20 {
            board.push(true);
        }
        else {
            board.push(false);
        }
    }

    board

}


/// Get the state of a cell of the board
/// An outside cell is dead...
fn get_cell_value(x: i32, y: i32, width: i32, height: i32, board: &Vec<bool>) -> bool {
    if x >= width {
        false
    }
    else if x < 0 {
        false
    }
    else if y < 0 {
        false
    }
    else if y >= height {
        false
    }
    else {
        board[(x + y * width) as usize]
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Game Of Live".to_owned(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}
