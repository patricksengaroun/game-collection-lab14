// Hunt the Wumpus: an 8-room cube, one pit, one bat room, one Wumpus. Provided code.
//
// TODO (Lab 14): refactor this into a 20-room cave stored as a table, with two pits and
// two bat rooms.
// TODO: remove the next line once every item below is used.
#![allow(dead_code)]

use crate::input::{read_int, read_line_trimmed};
use rand::RngExt;
use std::io::{self, Write};

// Provided for Lab 14. The new cave has 20 rooms.
const ROOM_COUNT: u32 = 20;
const ARROWS: u32 = 5;

// Exits of each room of the new cave. Room r is stored at index r - 1.
const CAVE: [[u32; 3]; 20] = [
    [2, 5, 8],
    [1, 3, 10],
    [2, 4, 12],
    [3, 5, 14],
    [1, 4, 6],
    [5, 7, 15],
    [6, 8, 17],
    [1, 7, 9],
    [8, 10, 18],
    [2, 9, 11],
    [10, 12, 19],
    [3, 11, 13],
    [12, 14, 20],
    [4, 13, 15],
    [6, 14, 16],
    [15, 17, 20],
    [7, 16, 18],
    [9, 17, 19],
    [11, 18, 20],
    [13, 16, 19],
];

// TODO: recursive. True if `target` appears in `values`.
pub fn contains(_values: &[u32], _target: u32) -> bool {
    todo!("Implement contains")
}

// TODO: recursive. True if no value appears twice in `values`. Use `contains`.
pub fn all_distinct(_values: &[u32]) -> bool {
    todo!("Implement all_distinct")
}

// TODO: replace this `match` with a lookup in CAVE. It should return [u32; 3].
pub fn exits(room: u32) -> (u32, u32, u32) {
    match room {
        1 => (2, 4, 5),
        2 => (1, 3, 6),
        3 => (2, 4, 7),
        4 => (1, 3, 8),
        5 => (1, 6, 8),
        6 => (2, 5, 7),
        7 => (3, 6, 8),
        8 => (4, 5, 7),
        _ => panic!("room {room} is not in the cave"),
    }
}

pub fn is_exit(room: u32, target: u32) -> bool {
    let (a, b, c) = exits(room);
    target == a || target == b || target == c
}

pub fn print_warnings(room: u32, pit: u32, bats: u32, wumpus: u32) {
    let (a, b, c) = exits(room);
    if a == pit || b == pit || c == pit {
        println!("You feel a draft.");
    }
    if a == bats || b == bats || c == bats {
        println!("You hear rustling.");
    }
    if a == wumpus || b == wumpus || c == wumpus {
        println!("You smell a Wumpus.");
    }
}

/// Returns (player, pit, bats, wumpus), all in different rooms.
pub fn place_hazards() -> (u32, u32, u32, u32) {
    let mut rng = rand::rng();
    loop {
        let player = rng.random_range(1..=8);
        let pit = rng.random_range(1..=8);
        let bats = rng.random_range(1..=8);
        let wumpus = rng.random_range(1..=8);
        if player != pit && player != bats && player != wumpus && pit != bats && pit != wumpus && bats != wumpus {
            return (player, pit, bats, wumpus);
        }
    }
}

pub fn shoot(target: u32, wumpus: u32) -> bool {
    target == wumpus
}

pub fn bat_destination() -> u32 {
    rand::rng().random_range(1..=8)
}

/// With probability 3/4 the Wumpus moves to a random exit; otherwise it stays.
pub fn wake_wumpus(wumpus: u32) -> u32 {
    let mut rng = rand::rng();
    if rng.random_range(0..4) < 3 {
        let (a, b, c) = exits(wumpus);
        match rng.random_range(0..3) {
            0 => a,
            1 => b,
            _ => c,
        }
    } else {
        wumpus
    }
}

fn read_exit(room: u32, prompt: &str) -> u32 {
    loop {
        let target = read_int(prompt, 1, 8);
        if is_exit(room, target) {
            return target;
        }
        println!("That is not an exit.");
    }
}

pub fn play() {
    let (mut player, pit, bats, mut wumpus) = place_hazards();
    let mut arrows = 3;
    println!("You enter the cave with {arrows} arrows.");
    loop {
        println!();
        let (a, b, c) = exits(player);
        println!("You are in room {player}. Exits: {a}, {b}, {c}.");
        print_warnings(player, pit, bats, wumpus);
        print!("Move or shoot (m/s)? ");
        io::stdout().flush().expect("failed to flush output");
        let action = read_line_trimmed().to_ascii_lowercase();
        if action == "m" {
            player = read_exit(player, "Move to room: ");
            if player == bats {
                println!("Bats carry you away!");
                player = bat_destination();
            }
            if player == pit {
                println!("You fell into a pit. You lose.");
                return;
            }
            if player == wumpus {
                println!("The Wumpus ate you. You lose.");
                return;
            }
        } else if action == "s" {
            let target = read_exit(player, "Shoot into room: ");
            if shoot(target, wumpus) {
                println!("You hit the Wumpus. You win.");
                return;
            }
            arrows -= 1;
            println!("You missed. Arrows left: {arrows}.");
            if arrows == 0 {
                println!("You are out of arrows. You lose.");
                return;
            }
            wumpus = wake_wumpus(wumpus);
            if wumpus == player {
                println!("The Wumpus found you. You lose.");
                return;
            }
        } else {
            println!("Enter m or s.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The exits of a room as a list of three. `.into()` accepts the old tuple and the new
    // array, so the supplied tests compile before and after you change `exits`.
    fn exits_of(room: u32) -> [u32; 3] {
        exits(room).into()
    }

    // Supplied by the instructor. They fail until `exits` reads the 20-room cave.
    #[test]
    fn cave_exits_in_range() {
        for room in 1..=ROOM_COUNT {
            for exit in exits_of(room) {
                assert!(exit >= 1 && exit <= ROOM_COUNT);
            }
        }
    }

    #[test]
    fn cave_is_symmetric() {
        for a in 1..=ROOM_COUNT {
            for b in exits_of(a) {
                assert!(exits_of(b).contains(&a), "room {b} does not lead back to room {a}");
            }
        }
    }

    // Update this test when `place_hazards` changes.
    #[test]
    fn hazards_are_distinct() {
        for _ in 0..200 {
            let (player, pit, bats, wumpus) = place_hazards();
            assert!(player != pit && player != bats && player != wumpus);
            assert!(pit != bats && pit != wumpus && bats != wumpus);
        }
    }

    // TODO: at least three tests of your own:
    //   one for `contains` (include the empty list),
    //   one for `all_distinct` (a list with a repeat and a list without),
    //   one for `exits` on a known room.
}
