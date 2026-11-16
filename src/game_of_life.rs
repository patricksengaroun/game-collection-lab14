// Game of Life, Weeks 12-13: grid, display, neighbor counts, the rule for one cell,
// generations, and preset patterns.
use crate::input::read_int;
use std::io;

const ROWS: usize = 10;
const COLS: usize = 20;
type Grid = [[bool; COLS]; ROWS];

pub fn new_grid() -> Grid {
    [[false; COLS]; ROWS]
}

pub fn print_grid(grid: &Grid) {
    for r in 0..ROWS {
        for c in 0..COLS {
            print!("{}", if grid[r][c] { '#' } else { '.' });
        }
        println!();
    }
}

// Neighbors of (row, col): the eight surrounding cells, excluding the cell itself.
// Cells beyond the edge count as dead.
pub fn count_neighbors(grid: &Grid, row: usize, col: usize) -> u8 {
    let mut count = 0;
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let r = row as i32 + dr;
            let c = col as i32 + dc;
            if r >= 0 && r < ROWS as i32 && c >= 0 && c < COLS as i32 && grid[r as usize][c as usize] {
                count += 1;
            }
        }
    }
    count
}

pub fn next_state(alive: bool, neighbors: u8) -> bool {
    match (alive, neighbors) {
        (true, 2) | (true, 3) => true,
        (false, 3) => true,
        _ => false,
    }
}

// The next generation. Reads from `grid` and writes to a new grid, so a cell is never
// judged using neighbors that have already been updated.
pub fn next_generation(grid: &Grid) -> Grid {
    let mut next = new_grid();
    for r in 0..ROWS {
        for c in 0..COLS {
            next[r][c] = next_state(grid[r][c], count_neighbors(grid, r, c));
        }
    }
    next
}

// 1 = blinker, 2 = block, anything else = glider.
pub fn load_pattern(choice: u32) -> Grid {
    let mut grid = new_grid();
    let cells: &[(usize, usize)] = match choice {
        1 => &[(4, 9), (4, 10), (4, 11)],
        2 => &[(4, 10), (4, 11), (5, 10), (5, 11)],
        _ => &[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)],
    };
    for &(r, c) in cells {
        grid[r][c] = true;
    }
    grid
}

// Number of live cells.
pub fn population(grid: &Grid) -> usize {
    let mut count = 0;
    for r in 0..ROWS {
        for c in 0..COLS {
            if grid[r][c] {
                count += 1;
            }
        }
    }
    count
}

pub fn play() {
    println!("1) Blinker  2) Block  3) Glider");
    let choice = read_int("Pattern: ", 1, 3);
    let generations = read_int("Generations: ", 0, 200);
    let mut grid = load_pattern(choice);
    for generation in 0..=generations {
        println!("Generation {generation}, population {}", population(&grid));
        print_grid(&grid);
        if generation < generations {
            println!("Press Enter for the next generation.");
            let mut wait = String::new();
            let bytes = io::stdin().read_line(&mut wait).expect("failed to read input");
            if bytes == 0 {
                return;
            }
            grid = next_generation(&grid);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid_from(cells: &[(usize, usize)]) -> Grid {
        let mut grid = new_grid();
        for &(r, c) in cells {
            grid[r][c] = true;
        }
        grid
    }

    #[test]
    fn live_1_dies() {
        assert!(!next_state(true, 1));
    }

    #[test]
    fn live_2_survives() {
        assert!(next_state(true, 2));
    }

    #[test]
    fn dead_3_born() {
        assert!(next_state(false, 3));
    }

    #[test]
    fn live_4_dies() {
        assert!(!next_state(true, 4));
    }

    #[test]
    fn dead_2_stays_dead() {
        assert!(!next_state(false, 2));
    }

    #[test]
    fn block_is_still() {
        let block = grid_from(&[(4, 10), (4, 11), (5, 10), (5, 11)]);
        assert_eq!(next_generation(&block), block);
    }

    #[test]
    fn blinker_returns_after_two() {
        let start = grid_from(&[(4, 10), (5, 10), (6, 10)]);
        let after_two = next_generation(&next_generation(&start));
        assert_eq!(after_two, start);
    }

    #[test]
    fn blinker_is_horizontal_after_one() {
        let start = grid_from(&[(4, 10), (5, 10), (6, 10)]);
        assert_eq!(next_generation(&start), grid_from(&[(5, 9), (5, 10), (5, 11)]));
    }

    #[test]
    fn glider_moves_one_down_one_right_after_four() {
        let mut grid = load_pattern(3);
        for _ in 0..4 {
            grid = next_generation(&grid);
        }
        assert_eq!(population(&grid), 5);
        assert_eq!(grid, grid_from(&[(2, 3), (3, 4), (4, 2), (4, 3), (4, 4)]));
    }
}
