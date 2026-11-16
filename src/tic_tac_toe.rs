// Tic-Tac-Toe: human is X, computer is O.
use crate::input::{read_int, read_line_trimmed};
use rand::RngExt;
use std::io::{self, Write};

pub type Board = [[char; 3]; 3];

pub fn new_board() -> Board {
    [[' '; 3]; 3]
}

pub fn print_board(board: &Board) {
    println!();
    for row in 0..3 {
        for col in 0..3 {
            let cell = board[row][col];
            if cell == ' ' {
                print!(" {} ", row * 3 + col + 1);
            } else {
                print!(" {cell} ");
            }
            if col < 2 {
                print!("|");
            }
        }
        println!();
        if row < 2 {
            println!("---+---+---");
        }
    }
    println!();
}

pub fn is_cell_empty(board: &Board, row: usize, col: usize) -> bool {
    board[row][col] == ' '
}

pub fn place_mark(board: &mut Board, row: usize, col: usize, mark: char) {
    board[row][col] = mark;
}

pub fn read_move(board: &Board) -> (usize, usize) {
    loop {
        let n = read_int("Your move (1-9): ", 1, 9) as usize;
        let row = (n - 1) / 3;
        let col = (n - 1) % 3;
        if is_cell_empty(board, row, col) {
            return (row, col);
        }
        println!("That cell is taken.");
    }
}

pub fn is_full(board: &Board) -> bool {
    for row in 0..3 {
        for col in 0..3 {
            if is_cell_empty(board, row, col) {
                return false;
            }
        }
    }
    true
}

fn line_winner(a: char, b: char, c: char) -> Option<char> {
    if a != ' ' && a == b && b == c { Some(a) } else { None }
}

pub fn winner(board: &Board) -> Option<char> {
    for i in 0..3 {
        if let Some(w) = line_winner(board[i][0], board[i][1], board[i][2]) {
            return Some(w);
        }
        if let Some(w) = line_winner(board[0][i], board[1][i], board[2][i]) {
            return Some(w);
        }
    }
    if let Some(w) = line_winner(board[0][0], board[1][1], board[2][2]) {
        return Some(w);
    }
    line_winner(board[0][2], board[1][1], board[2][0])
}

pub fn computer_move(board: &Board) -> (usize, usize) {
    let mut rng = rand::rng();
    loop {
        let row = rng.random_range(0..3);
        let col = rng.random_range(0..3);
        if is_cell_empty(board, row, col) {
            return (row, col);
        }
    }
}

fn play_one_game() {
    let mut board = new_board();
    let mut human_turn = rand::rng().random_range(0..2) == 0;
    println!("{} goes first.", if human_turn { "You" } else { "The computer" });
    loop {
        print_board(&board);
        if human_turn {
            let (row, col) = read_move(&board);
            place_mark(&mut board, row, col, 'X');
        } else {
            let (row, col) = computer_move(&board);
            println!("The computer plays {}.", row * 3 + col + 1);
            place_mark(&mut board, row, col, 'O');
        }
        match winner(&board) {
            Some('X') => {
                print_board(&board);
                println!("You win!");
                return;
            }
            Some(_) => {
                print_board(&board);
                println!("Computer wins.");
                return;
            }
            None => {}
        }
        if is_full(&board) {
            print_board(&board);
            println!("Draw.");
            return;
        }
        human_turn = !human_turn;
    }
}

pub fn play() {
    loop {
        play_one_game();
        print!("Play again? (y/n): ");
        io::stdout().flush().expect("failed to flush output");
        if !read_line_trimmed().eq_ignore_ascii_case("y") {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board_from(rows: [&str; 3]) -> Board {
        let mut board = new_board();
        for (r, row) in rows.iter().enumerate() {
            for (c, ch) in row.chars().enumerate() {
                board[r][c] = ch;
            }
        }
        board
    }

    // Supplied by the instructor.
    #[test]
    fn x_wins_a_row() {
        assert_eq!(winner(&board_from(["XXX", "OO ", "   "])), Some('X'));
    }

    #[test]
    fn o_wins_a_diagonal() {
        assert_eq!(winner(&board_from(["OXX", "XO ", "  O"])), Some('O'));
    }

    #[test]
    fn full_board_without_winner() {
        let board = board_from(["XOX", "XOO", "OXX"]);
        assert_eq!(winner(&board), None);
        assert!(is_full(&board));
    }

    // Written by the student.
    #[test]
    fn x_wins_a_column() {
        assert_eq!(winner(&board_from(["XO ", "XO ", "X  "])), Some('X'));
    }

    #[test]
    fn empty_board_has_no_winner() {
        let board = new_board();
        assert_eq!(winner(&board), None);
        assert!(!is_full(&board));
    }

    #[test]
    fn nearly_full_board_has_no_winner() {
        let board = board_from(["XOX", "XOO", "OX "]);
        assert_eq!(winner(&board), None);
        assert!(!is_full(&board));
    }
}
