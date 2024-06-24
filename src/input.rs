use std::io::{self, Write};
use rand::Rng;
use crate::board::BOARD_SIZE;


pub fn get_player_input() -> (usize, usize) {
    loop {
        print!("\x1b[1;37mEnter coordinates to fire (row, col): \x1b[0m");
        io::stdout().flush().unwrap();
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        let coordinates: Vec<Result<usize, _>> = input.trim()
            .split(',')
            .map(|s: &str| s.trim().parse())
            .collect();

        if coordinates.len() == 2 {
            if let (Ok(row), Ok(col)) = (coordinates[0].clone(), coordinates[1].clone()) {
                if row < BOARD_SIZE && col < BOARD_SIZE {
                    return (row, col);
                }
            }
        }
        println!("\x1b[1;31mInvalid input. Please enter row and column numbers separeted by comma.\x1b[0bm")
    }
}


pub fn generate_opponent_move() -> (usize, usize){
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..BOARD_SIZE), rng.gen_range(0..BOARD_SIZE))
}

pub fn get_player_name(player_id: u8) -> String {
    print!("Enter name for player {} ", player_id);

    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed to read line");
    name.trim().to_string()
}