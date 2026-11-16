// Battleship on an 8x8 board against a computer opponent.
use crate::input::{read_int, read_line_trimmed};
use rand::RngExt;
use std::io::{self, Write};

const SIZE: usize = 8;
const FLEET: [usize; 2] = [3, 2];

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Water,
    Ship,
    Hit,
    Miss,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Shot {
    Hit,
    Miss,
    Repeat,
}

pub type Board = [[Cell; SIZE]; SIZE];

pub fn new_board() -> Board {
    [[Cell::Water; SIZE]; SIZE]
}

pub fn parse_coord(text: &str) -> Option<(usize, usize)> {
    let mut chars = text.trim().chars();
    let letter = chars.next()?.to_ascii_uppercase();
    let digit = chars.next()?;
    if chars.next().is_some() || !('A'..='H').contains(&letter) {
        return None;
    }
    let col = digit.to_digit(10)? as usize;
    if col < 1 || col > SIZE {
        return None;
    }
    Some(((letter as u8 - b'A') as usize, col - 1))
}

pub fn can_place(board: &Board, row: usize, col: usize, len: usize, horizontal: bool) -> bool {
    for i in 0..len {
        let (r, c) = if horizontal { (row, col + i) } else { (row + i, col) };
        if r >= SIZE || c >= SIZE || board[r][c] != Cell::Water {
            return false;
        }
    }
    true
}

pub fn place_ship(board: &mut Board, row: usize, col: usize, len: usize, horizontal: bool) {
    for i in 0..len {
        if horizontal {
            board[row][col + i] = Cell::Ship;
        } else {
            board[row + i][col] = Cell::Ship;
        }
    }
}

pub fn print_board(board: &Board, reveal_ships: bool) {
    print!("  ");
    for col in 1..=SIZE {
        print!("{col} ");
    }
    println!();
    for row in 0..SIZE {
        print!("{} ", (b'A' + row as u8) as char);
        for col in 0..SIZE {
            let symbol = match board[row][col] {
                Cell::Water => '~',
                Cell::Ship => if reveal_ships { 'S' } else { '~' },
                Cell::Hit => 'X',
                Cell::Miss => 'o',
            };
            print!("{symbol} ");
        }
        println!();
    }
}

pub fn read_placement(board: &Board, len: usize) -> (usize, usize, bool) {
    loop {
        print!("Start coordinate for the ship of length {len} (for example B3): ");
        io::stdout().flush().expect("failed to flush output");
        let text = read_line_trimmed();
        let (row, col) = match parse_coord(&text) {
            Some(rc) => rc,
            None => {
                println!("That is not a coordinate. Use a letter A-H and a digit 1-8.");
                continue;
            }
        };
        print!("Horizontal or vertical (H/V): ");
        io::stdout().flush().expect("failed to flush output");
        let horizontal = match read_line_trimmed().to_ascii_uppercase().as_str() {
            "H" => true,
            "V" => false,
            _ => {
                println!("Enter H or V.");
                continue;
            }
        };
        let (last_row, last_col) = if horizontal { (row, col + len - 1) } else { (row + len - 1, col) };
        if last_row >= SIZE || last_col >= SIZE {
            println!("The ship does not fit on the board there.");
            continue;
        }
        if !can_place(board, row, col, len, horizontal) {
            println!("The ship overlaps another ship.");
            continue;
        }
        return (row, col, horizontal);
    }
}

pub fn place_fleet_randomly(board: &mut Board) {
    let mut rng = rand::rng();
    for len in FLEET {
        loop {
            let row = rng.random_range(0..SIZE);
            let col = rng.random_range(0..SIZE);
            let horizontal = rng.random_range(0..2) == 0;
            if can_place(board, row, col, len, horizontal) {
                place_ship(board, row, col, len, horizontal);
                break;
            }
        }
    }
}

pub fn fire(board: &mut Board, row: usize, col: usize) -> Shot {
    match board[row][col] {
        Cell::Water => {
            board[row][col] = Cell::Miss;
            Shot::Miss
        }
        Cell::Ship => {
            board[row][col] = Cell::Hit;
            Shot::Hit
        }
        Cell::Hit | Cell::Miss => Shot::Repeat,
    }
}

pub fn ships_remaining(board: &Board) -> usize {
    let mut count = 0;
    for row in 0..SIZE {
        for col in 0..SIZE {
            if board[row][col] == Cell::Ship {
                count += 1;
            }
        }
    }
    count
}

pub fn computer_fire(board: &Board) -> (usize, usize) {
    let mut rng = rand::rng();
    loop {
        let row = rng.random_range(0..SIZE);
        let col = rng.random_range(0..SIZE);
        if board[row][col] != Cell::Hit && board[row][col] != Cell::Miss {
            return (row, col);
        }
    }
}

pub fn read_target(board: &Board) -> (usize, usize) {
    let _ = board;
    loop {
        print!("Fire at (for example C5): ");
        io::stdout().flush().expect("failed to flush output");
        match parse_coord(&read_line_trimmed()) {
            Some(rc) => return rc,
            None => println!("That is not a coordinate. Use a letter A-H and a digit 1-8."),
        }
    }
}

fn label(row: usize, col: usize) -> String {
    format!("{}{}", (b'A' + row as u8) as char, col + 1)
}

pub fn play() {
    let mut player = new_board();
    let mut computer = new_board();

    println!("1) Place your ships yourself  2) Place them automatically");
    if read_int("Choice (default 2): ", 1, 2) == 1 {
        for len in FLEET {
            print_board(&player, true);
            let (row, col, horizontal) = read_placement(&player, len);
            place_ship(&mut player, row, col, len, horizontal);
        }
    } else {
        place_fleet_randomly(&mut player);
    }
    place_fleet_randomly(&mut computer);

    loop {
        println!();
        println!("Computer's board:");
        print_board(&computer, false);
        println!("Your board:");
        print_board(&player, true);

        loop {
            let (row, col) = read_target(&computer);
            match fire(&mut computer, row, col) {
                Shot::Hit => {
                    println!("You fire at {}: Hit!", label(row, col));
                    break;
                }
                Shot::Miss => {
                    println!("You fire at {}: Miss.", label(row, col));
                    break;
                }
                Shot::Repeat => println!("You already fired there. Choose another cell."),
            }
        }
        if ships_remaining(&computer) == 0 {
            println!("You sank the whole fleet. You win!");
            return;
        }

        let (row, col) = computer_fire(&player);
        match fire(&mut player, row, col) {
            Shot::Hit => println!("The computer fires at {}: Hit!", label(row, col)),
            _ => println!("The computer fires at {}: Miss.", label(row, col)),
        }
        if ships_remaining(&player) == 0 {
            println!("The computer sank your fleet. You lose.");
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_coord_accepts_both_cases() {
        assert_eq!(parse_coord("B7"), Some((1, 6)));
        assert_eq!(parse_coord("b7"), Some((1, 6)));
        assert_eq!(parse_coord("I1"), None);
        assert_eq!(parse_coord("A9"), None);
        assert_eq!(parse_coord("A"), None);
    }

    #[test]
    fn placement_rules() {
        let mut board = new_board();
        assert!(can_place(&board, 0, 0, 3, true));
        assert!(!can_place(&board, 0, 6, 3, true));
        place_ship(&mut board, 0, 0, 3, true);
        assert!(!can_place(&board, 0, 2, 2, false));
        assert_eq!(ships_remaining(&board), 3);
    }

    #[test]
    fn firing_changes_the_board() {
        let mut board = new_board();
        place_ship(&mut board, 2, 2, 2, true);
        assert_eq!(fire(&mut board, 2, 2), Shot::Hit);
        assert_eq!(fire(&mut board, 2, 2), Shot::Repeat);
        assert_eq!(fire(&mut board, 5, 5), Shot::Miss);
        assert_eq!(ships_remaining(&board), 1);
    }
}
