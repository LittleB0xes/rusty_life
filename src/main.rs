use macroquad::prelude::*;
use macroquad::rand;
use macroquad::rand::rand;

use std::time::{SystemTime, UNIX_EPOCH};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const MARGIN: usize = 0;

#[macroquad::main(window_conf)]
async fn main() {


    // General setting
    
    let mut paused = true; 

    let mut frame_count: u32 = 0;
    let mut board: Vec<bool>;
    let mut temp_board: Vec<bool>;
    let mut cell_size = 4.0;
    let cell_size_limit = (4.0, 20.0);

    let mut camera = Vec2::ZERO;

    let width = WIDTH / cell_size as usize;
    let height = (HEIGHT - MARGIN) / cell_size as usize;

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

            // Copy temporary board to active board
            board = temp_board.clone();

        }
        // If paused, draw or remove cells
            mouse_cell.x = ((mouse_position().0 - 0.5 * cell_size + camera.x) / cell_size).round();
            mouse_cell.y = ((mouse_position().1 - 0.5 * cell_size + camera.y) / cell_size).round();
            if is_mouse_button_pressed(MouseButton::Left) && paused {
                let index = (mouse_cell.x + mouse_cell.y * width as f32) as usize;
                board[index] = !board[index];
            }


        // Draw board
        for (index, cell) in board.iter().enumerate() {
            if *cell {
                let mut color: Color = WHITE;
                if paused {color = GRAY}
                draw_circle_lines(
                    0.5 * cell_size - camera.x + (index % width) as f32 * cell_size,
                    0.5 * cell_size - camera.y + (index / width) as f32 * cell_size,
                    0.4 * cell_size,
                    cell_size / 6.0,
                    color);
            }
        }
        // Show some data
        //draw_text(&format!("{}", get_fps()), 10.0, 700.0, 16.0, WHITE);

        // Check inputs
        if is_key_pressed(KeyCode::R) {
            board = randomize_board(width, height);
        }
        else if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        else if is_key_pressed(KeyCode::C) {
            board = clear_board(width, height);
        }
        else if is_key_pressed(KeyCode::D) {
            cell_size += 1.0;
            if cell_size > cell_size_limit.1 {cell_size = cell_size_limit.1}
        }
        else if is_key_pressed(KeyCode::S) {
            cell_size -= 1.0;
            if cell_size < cell_size_limit.0 {cell_size = cell_size_limit.0}
        }


        // Camera motion
        if is_key_down(KeyCode::Up) {
            camera.y -= cell_size as f32;
        }
        else if is_key_down(KeyCode::Down) {
            camera.y += cell_size as f32;
        }

        if is_key_down(KeyCode::Left) {
            camera.x -= cell_size as f32;
        }
        else if is_key_down(KeyCode::Right) {
            camera.x += cell_size as f32;
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
    let mut board = vec![false; width * height];

    // Seed the random generator
    rand::srand(SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis() as u64);
    for i in 0..(width * height) {
        if rand() % 100 < 20 {
            board[i] = true
        }
    }

    board

}


/// Get the state of a cell of the board
/// An outside cell is dead...
fn get_cell_value(mut x: i32, mut y: i32, width: i32, height: i32, board: &Vec<bool>) -> bool {

    // Torus world up - bottom connexion and left-right connexion
    if x >= width {
        x = 0;
    }
    else if x < 0 {
        x = width - 1;
    }
    if y >= height {
        y = 0;
    }
    else if y < 0 {
        y = height - 1;
    }

    board[(x + y * width) as usize]

    // Ended world with definitly dead cells around it
    //if x >= width || x < 0 || y < 0 || y >= height {
    //    false
    //}
    //else {
    //    board[(x + y * width) as usize]
    //}
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Game Of Live".to_owned(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}
