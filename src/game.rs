use std::io;
use crate::board::{Board, CellState};
use crate::input::{get_player_input, get_player_name};



pub fn start_game() {

    let player_name = get_player_name(1);
    let opponent_name = get_player_name(2);


    let mut player_board = Board::new();
    let mut opponent_board = Board::new();
    let mut counter = 0;

    place_ships(&mut player_board);
    place_ships(&mut opponent_board);

    loop {
        clear_screen();

        display_board(&player_board, &opponent_board, &player_name, &opponent_name, check_player_turn(counter));

        if handle_player_turn(&mut opponent_board, &player_name) {
            break;
        }

        if handle_active_opponent_turn(&mut player_board, &opponent_name) {
            break;
        }
        counter += 1;
    }


    fn handle_player_turn(opponent_board: &mut Board, player_name: &str) -> bool {
        let (player_row, player_col) = get_player_input();
        let result = opponent_board.fire(player_row, player_col);

        match result {
            CellState::Miss =>  println!("\x1b[36m{} missed!\x1b[0m", player_name),
            CellState::Hit =>   println!("\x1b[31m{} hit one of your ships!\x1b[0m", player_name),
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


    fn handle_active_opponent_turn(player_board: &mut Board, opponent_name: &str) -> bool {

        let (opponent_row, opponent_col) = get_player_input();
        let result = player_board.fire(opponent_row, opponent_col);
        match result {
            CellState::Miss =>  println!("\x1b[36m{} missed!\x1b[0m", opponent_name),
            CellState::Hit =>   println!("\x1b[31m{} hit one of your ships!\x1b[0m", opponent_name),
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

    /*
    fn handle_opponent_turn(player_board: &mut Board, opponent_name: &str) -> bool {

        let (opponent_row, opponent_col) = generate_opponent_move();
        let result = player_board.fire(opponent_row, opponent_col);
        match result {
            CellState::Miss =>  println!("\x1b[36m{} missed!\x1b[0m", opponent_name),
            CellState::Hit =>   println!("\x1b[31m{} hit one of your ships!\x1b[0m", opponent_name),
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
    */


    fn check_player_turn(player_turn: i32) -> bool {
       if player_turn % 2 == 0 || player_turn == 0 {
           return false
       }
       return true;
    }


    fn display_board(player_board: &Board, opponent_board: &Board, player_name: &str, opponent_name: &str, check_player_turn: bool) {
        println!("\x1b[1;37m{} board:\x1b[0m", player_name);
        player_board.display(check_player_turn);

        println!("\x1b[1;37m{} board:\x1b[0m", opponent_name);
        opponent_board.display(!check_player_turn);
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