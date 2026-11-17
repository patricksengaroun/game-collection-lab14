// Hunt the Wumpus: an 8-room cube, one pit, one bat room, one Wumpus. Provided code.
//
// TODO (Lab 14): refactor this into a 20-room cave stored as a table, with two pits and
// two bat rooms.
// TODO: remove the next line once every item below is used.
#![allow(dead_code)]

use crate::input::read_int;
use rand::RngExt;

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
pub fn contains(values: &[u32], target: u32) -> bool {
    if values.is_empty() {
        return false;
    }

    if values[0] == target {
        return true;
    }

    contains(&values[1..], target)
}

// TODO: recursive. True if no value appears twice in `values`. Use `contains`.
pub fn all_distinct(values: &[u32]) -> bool {
    match values.split_first() {
        None => true,
        Some((first, rest)) => {
            if rest.contains(first) {
                false
            } else {
                all_distinct(rest)
            }
        }
    }
}

// TODO: replace this `match` with a lookup in CAVE. It should return [u32; 3].
pub fn exits(room: u32) -> [u32; 3] {
    CAVE[(room - 1) as usize]
}

pub fn is_exit(room: u32, target: u32) -> bool {
    contains(&exits(room), target)
}

pub fn print_warnings(room: u32, pit: &[u32], bats: &[u32], wumpus: u32) {
    let mut near_pit = false;
    let mut near_bats = false;
    let mut near_wumpus = false;

    for exit in exits(room) {
        if contains(pit, exit) {
            near_pit = true;
        }
        if contains(bats, exit) {
            near_bats = true;
        }
        if exit == wumpus {
            near_wumpus = true;
        }
    }

    if near_pit {
        println!("You feel a draft.");
    }
    if near_bats {
        println!("You hear rustling.");
    }
    if near_wumpus {
        println!("You smell a Wumpus.");
    }
}

/// Returns (player, pit, bats, wumpus), all in different rooms.
pub fn place_hazards() -> (u32, [u32; 2], [u32; 2], u32) {
    let mut rng = rand::rng();
    
    loop {
        let player = rng.random_range(1..=ROOM_COUNT);
        let pit1 = rng.random_range(1..=ROOM_COUNT);
        let pit2 = rng.random_range(1..=ROOM_COUNT);
        let bat1 = rng.random_range(1..=ROOM_COUNT);
        let bat2 = rng.random_range(1..=ROOM_COUNT);
        let wumpus = rng.random_range(1..=ROOM_COUNT);

        let rooms = [player, pit1, pit2, bat1, bat2, wumpus];

        if all_distinct(&rooms) {
            return (player, [pit1, pit2], [bat1, bat2], wumpus);
        }
    }
}

pub fn shoot(target: u32, wumpus: u32) -> bool {
    target == wumpus
}

pub fn bat_destination() -> u32 {
    let mut rng = rand::rng();
    rng.random_range(1..=ROOM_COUNT)
}

/// With probability 3/4 the Wumpus moves to a random exit; otherwise it stays.
pub fn wake_wumpus(wumpus_room: u32, exits: [u32; 3]) -> u32 {
    let mut rng = rand::rng();
 
    if rng.random_bool(0.75) {
        let random_exit_index = rng.random_range(0..3);
        exits[random_exit_index]
    } else {
        wumpus_room
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
    let (mut player_room, pits, bats, wumpus_room) = place_hazards();
    let mut arrows_left = ARROWS;
    println!("You enter the cave with {ARROWS} arrows.");
    
    loop {

        loop {
            if bats.contains(&player_room) {
                println!("A giant bat grabs you and flies you away!");
                player_room = bat_destination();
                continue; 
            }
            if pits.contains(&player_room) {
                println!("Yikes! You fell into a bottomless pit! Game over.");
                return;
            }
            if player_room == wumpus_room {
                println!("You bumped into the Wumpus! Game over.");
                return;
            }
            break;
        }
        
        println!("\nYou are in room {player_room}.");

        let choice = read_int("Do you want to Move (1) or Shoot (2)? ", 1, 2); 

        if choice == 1 {
            player_room = read_int("Where to? ", 1, ROOM_COUNT);
        } else if choice == 2 {
            let target_room = read_int("Where to? ", 1, ROOM_COUNT);
            
            if target_room == wumpus_room {
                println!("You shot the Wumpus! You win!");
                return;
            } else {
                println!("Your arrow missed!");
                arrows_left -= 1;
                if arrows_left == 0 {
                    println!("You ran out of arrows! Game over.");
                    return;
                }
            }
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
        for _ in 0..100 {
            let (player, pits, bats, wumpus) = place_hazards();
            let all_rooms = [player, pits[0], pits[1], bats[0], bats[1], wumpus];
            
            assert!(all_distinct(&all_rooms), "Hazards are not distinct: {:?}", all_rooms);
            
            for &room in &all_rooms {
                assert!(
                    room >= 1 && room <= ROOM_COUNT, 
                    "Room {} is out of bounds (1-{})", room, ROOM_COUNT
                );
            }
        }
    }
}
    // TODO: at least three tests of your own:
    //   one for `contains` (include the empty list),
    //   one for `all_distinct` (a list with a repeat and a list without),
    //   one for `exits` on a known room.
    
    #[test]
    fn test_contains_empty_end_and_missing() {
        let empty_list: [u32; 0] = [];
        assert!(!empty_list.contains(&5), "Empty list should not contain anything");

        let list_with_end = [1, 2, 3, 4, 5];
        assert!(list_with_end.contains(&5), "Should find target at the end of the list");

        let list_missing = [1, 2, 3, 4];
        assert!(!list_missing.contains(&9), "Should return false if target is missing");
    }

    #[test]
    fn test_all_distinct_cases() {
        assert!(all_distinct(&[]), "Empty list must be distinct");

        assert!(all_distinct(&[42]), "Single item list must be distinct");

        assert!(all_distinct(&[1, 2, 3, 4, 5]), "List without repeats should return true");

        assert!(!all_distinct(&[1, 2, 3, 4, 5, 1]), "Repeat at the end should return false");
    }

    #[test]
    fn test_exits_for_known_room() {
        let known_room = 1;

        let expected_exits = [2, 5, 8];

        let actual_exits = exits(known_room);

        assert_eq!(
            actual_exits, expected_exits,
            "Exits for room {} do not match the expected map layout", known_room
        );
    }
