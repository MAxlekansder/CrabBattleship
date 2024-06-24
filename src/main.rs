
use std::io::{self, Write};
use rand::Rng;

const BOARD_SIZE: usize = 10;       // defining the size of the game board

struct Board{                       // structuring the board
    grid: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    ships: Vec<(usize, usize)>,
}

#[derive(Clone, Copy, PartialEq)]
enum CellState {                    // different types of states
    Empty,
    Ship,
    Hit,
    Miss,
}


impl Board {
    fn new() -> Self{               // constructor where we set the board
        Board{
            grid: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            ships: Vec::new(),
        }
    }

    fn place_ship(&mut self, size: usize) {
        let mut rng = rand::thread_rng();

        loop {
            let row = rng.gen_range(0..BOARD_SIZE);
            let col = rng.gen_range(0.. BOARD_SIZE);
            let direction = rng.gen::<bool>();

            if self.can_place_ship(row, col, size, direction) {
                for i in 0..size {
                    let (r, c) = if direction { (row, col + i) } else { (row + i, col) };

                    self.grid[r][c] == CellState::Ship;
                    self.ships.push((r,c))
                }
                break;
            }

        }
    }


    fn can_place_ship(&self, row: usize, col: usize, size: usize, direction: bool) -> bool {       // check if ship can be placed
        if direction {
            if col + size > BOARD_SIZE { return false; }
            for i in 0..size {
                if self.grid[row][col + i] != CellState::Empty { return false; }
            }
        } else {
            if row + size > BOARD_SIZE { return false; }
            for i in 0..size {
                if self.grid[row + i][col] != CellState::Empty { return false; }
            }
        }
        true
    }



    // method to fire
    fn fire(&mut self, row: usize, col:usize) -> CellState {
        match self.grid[row][col] {
            CellState::Empty => { self.grid[row][col] = CellState::Miss;     // update the cell to "Miss" if cells empty
                CellState::Miss
            },
            CellState::Ship => { self.grid[row][col] = CellState::Hit;       // update cell to "Hit" if ship is hit
                CellState::Hit
            },
            _ => CellState::Miss,                                            // returns a miss if cell already been fired on
        }
    }


    fn display(&self, hide_ships: bool) {
        print!("   ");
        for i in 0..BOARD_SIZE { print!(" {} ", i)}
        println!();

        for (i, row) in self.grid.iter().enumerate() {
            print!("{:2}", i);
            for cell in row {
                match cell {
                    CellState::Empty => { if hide_ships { print!("   "); } else { print!(" \u{25a1} "); }}

                    CellState::Ship => { if hide_ships { print!("   ")} else { print!(" \u{25A0} ")}}

                    CellState::Hit => print!("\x1b[31m \u{25CF} \x1b[0m"),

                    CellState::Miss => print!("\x1b[36m \u{00B7} \x1b[0m"),
                }
            }
            println!();;
        }
    }

    fn is_game_over(&self) -> bool {
        self.ships.iter().all(|&(r,c)| self.grid[r][c] == CellState::Hit)
    }
}


fn main() {
    loop {

    }
}




// method to display the game board

// method for game over

fn get_player_input() {

}

fn generate_opponent_move() {

}
