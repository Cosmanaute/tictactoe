use std::io::{self, Write};

struct Board {
    grid: [[char; 3]; 3],
}

impl Board {
    fn new() -> Board {
        Board {
            grid: [[' '; 3]; 3],
        }
    }

    fn display(&self) {
        println!("  1 2 3");
        for (i, row) in self.grid.iter().enumerate() {
            print!("{} ", i + 1);
            for col in row.iter() {
                print!("{} ", col);
            }
            println!("");
        }
    }

    fn is_empty(&self, row: usize, col: usize) -> bool {
        self.grid[row][col] == ' '
    }

    fn place_mark(&mut self, row: usize, col: usize, mark: char) {
        self.grid[row][col] = mark;
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn check_win(&self, mark: char) -> bool {
        for row in self.grid.iter() {
            if row[0] == mark && row[1] == mark && row[2] == mark {
                return true;
            }
        }

        for col in 0..3 {
            if self.grid[0][col] == mark && self.grid[1][col] == mark && self.grid[2][col] == mark {
                return true;
            }
        }

        if self.grid[0][0] == mark && self.grid[1][1] == mark && self.grid[2][2] == mark {
            return true;
        }
        if self.grid[0][2] == mark && self.grid[1][1] == mark && self.grid[2][0] == mark {
            return true;
        }

        false
    }
}

fn main() {
    let mut board = Board::new();
    let mut current_player = 'X';

    println!("Welcome to Tic Tac Toe!");
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    loop {
        board.display();

        print!("Player {}, enter your move (row column): ", current_player);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let coords: Vec<usize> = input.split_whitespace()
            .map(|x| x.parse().expect("Invalid input"))
            .collect();

        if coords.len() != 2 {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("Invalid input");
            continue;
        }

        let row = coords[0] - 1;
        let col = coords[1] - 1;

        if row >= 3 || col >= 3 {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("Invalid move");
            continue;
        }

        if !board.is_empty(row, col) {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("Cell already occupied");
            continue;
        }

        board.place_mark(row, col, current_player);

        if board.check_win(current_player) {
            println!("Player {} wins!", current_player);
            break;
        }

        if current_player == 'X' {
           current_player = 'O';
        } else {
            current_player = 'X';
        }

        let mut is_tie = true;
        for row in 0..3 {
            for col in 0..3 {
                if board.is_empty(row, col) {
                    is_tie = false;
                    break;
                }
            }
        }
        if is_tie {
            println!("It's a tie!");
            break;
        }
    }
}
