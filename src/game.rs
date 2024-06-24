use std::io;
use crate::board::{Board, CellState};
use crate::input::{get_player_input, generate_opponent_move};

pub fn start_game() {
    let mut player_board = Board::new();
    let mut opponent_board = Board::new();

    place_ships(&mut player_board);
    place_ships(&mut opponent_board);

    loop {
        clear_screen();

        display_board(&player_board, &opponent_board);

        if handle_opponent_turn(&mut opponent_board) {
            break;
        }

        if handle_player_turn(&mut player_board) {
            break;
        }
    }


    fn handle_player_turn(opponent_board: &mut Board) -> bool {
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
            return true;
        }
        false
    }


    fn handle_opponent_turn(player_board: &mut Board) -> bool {

        let (opponent_row, opponent_col) = generate_opponent_move();
        let result = player_board.fire(opponent_row, opponent_col);
        match result {
            CellState::Miss => println!("\x1b[36mOpponent missed!\x1b[0m"),
            CellState::Hit => eprintln!("\x1b[31mThe opponent hit one of your ships!\x1b[0m"),
            _ => (),
        }

        println!("Press enter to continue...");
        io::stdin().read_line(&mut String::new()).expect("Failed to read input");

        if player_board.is_game_over() {
            println!("\x1b[1;31mYou lost the game!\x1b[0m");
            return true;
        }
        false
    }


    fn display_board(player_board: &Board, opponent_board: &Board) {
        println!("\x1b[1;37mYour board:\x1b[0m");
        player_board.display(false);
        println!("\x1b[1;37mOpponents board:\x1b[0m");
        opponent_board.display(true);
    }


    fn place_ships(board: &mut Board) {
        board.place_ship(5);
        board.place_ship(4);
        board.place_ship(3);
        board.place_ship(2);
    }


    fn clear_screen() {
        print!("\x1b[2J\x1b[1;1H");
    }
}