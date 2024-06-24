use std::io;
use crate::board::{Board, CellState};
use crate::input::{get_player_input, generate_opponent_move};

pub fn start_game() {
    let mut player_board = Board::new();
    let mut opponent_board = Board::new();

    player_board.place_ship(5);
    player_board.place_ship(4);
    player_board.place_ship(3);
    player_board.place_ship(2);

    opponent_board.place_ship(5);
    opponent_board.place_ship(4);
    opponent_board.place_ship(3);
    opponent_board.place_ship(2);


    loop {
        print!("\x1b[2J\x1b[1;1H");

        println!("\x1b[1;37mYour board:\x1b[0m");
        player_board.display(false);
        println!("\x1b[1;37mOpponents board:\x1b[0m");
        opponent_board.display(true);

        let (player_row, player_col) = get_player_input();
        let result = opponent_board.fire(player_row, player_col);

        match result {
            CellState::Miss => println!("\x1b[36mYou missed!\x1b[0m"),
            CellState::Hit => eprintln!("\x1b[31mYou hit a ship!\x1b[0m"),
            _ => (),
        }

        println!("Press enter to continue...");
        io::stdin().read_line(&mut String::new()).expect("failed to read line");

        if opponent_board.is_game_over() {
            println!("\x1b[1;32mYou won the game!\x1b[0m");
            break;
        }

        if player_board.is_game_over() {
            println!("\x1b[1;31mYou lost the game!\x1b[0m");
            break;
        }

        let (opponent_row, opponent_col) = generate_opponent_move();
        let result = player_board.fire(opponent_row, opponent_col);
        match result {
            CellState::Miss => println!("\x1b[36mOpponent missed!\x1b[0m"),
            CellState::Hit => eprintln!("\x1b[31mThe opponent hit one of your ships!\x1b[0m"),
            _ => (),
        }

        println!("Press enter to continue...");
        io::stdin().read_line(&mut String::new()).expect("Failed to read input");

    }

}