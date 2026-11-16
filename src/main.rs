mod battleship;
mod bulls_and_cows;
mod game_of_life;
mod input;
mod mad_libs;
mod tic_tac_toe;
mod wumpus;

use input::read_int;

fn print_title() {
    println!("  ____    _    __  __ _____    ____ ___  _     _     _____ ____ _____ ___ ___  _   _ ");
    println!(" / ___|  / \\  |  \\/  | ____|  / ___/ _ \\| |   | |   | ____/ ___|_   _|_ _/ _ \\| \\ | |");
    println!("| |  _  / _ \\ | |\\/| |  _|   | |  | | | | |   | |   |  _|| |     | |  | | | | |  \\| |");
    println!("| |_| |/ ___ \\| |  | | |___  | |__| |_| | |___| |___| |__| |___  | |  | | |_| | |\\  |");
    println!(" \\____/_/   \\_\\_|  |_|_____|  \\____\\___/|_____|_____|_____\\____| |_| |___\\___/|_| \\_|");
}

fn print_menu() {
    println!();
    println!("1. Mad Libs");
    println!("2. Hunt the Wumpus");
    println!("3. Bulls and Cows");
    println!("4. Tic-Tac-Toe");
    println!("5. Battleship");
    println!("6. Game of Life");
    println!("0. Quit");
}

fn main() {
    print_title();
    loop {
        print_menu();
        match read_int("Choice: ", 0, 6) {
            1 => mad_libs::play(),
            2 => wumpus::play(),
            3 => bulls_and_cows::play(),
            4 => tic_tac_toe::play(),
            5 => battleship::play(),
            6 => game_of_life::play(),
            _ => break,
        }
    }
    println!("Goodbye.");
}
