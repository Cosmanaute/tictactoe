use std::slice::SliceIndex;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;

fn draw_board(board: &[char; 9]) {
    println!("{}|{}|{}", board[0], board[1], board[2]);
    println!("{}|{}|{}", board[3], board[4], board[5]);
    println!("{}|{}|{}", board[6], board[7], board[8]);
}

fn get_input() -> u8 {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error reading input!");
    return input.parse::<i8>().unwrap();
}

fn main() {
    
    let mut board: [char; 9] = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
    let mut turn: char = 'o';

    for i in 0..10 {
        draw_board(&board);
        println!("{}'s turn to move: ", turn);
        let input = get_input();
        if board[input] == ' ' {
           board[input] == turn;
        }
        else {
            println!("Cannot move here!");
        }

    }

}

