// Bulls and Cows: guess a secret 4-digit code with no repeated digits.
use crate::input::read_line_trimmed;
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use std::io::{self, Write};

const DEBUG: bool = false;
const MAX_ATTEMPTS: u32 = 10;

pub fn has_duplicate(code: &[u8; 4]) -> bool {
    for i in 0..4 {
        for j in (i + 1)..4 {
            if code[i] == code[j] {
                return true;
            }
        }
    }
    false
}

pub fn generate_secret(rng: &mut StdRng) -> [u8; 4] {
    loop {
        let mut code = [0u8; 4];
        for digit in code.iter_mut() {
            *digit = rng.random_range(0..10);
        }
        if !has_duplicate(&code) {
            return code;
        }
    }
}

/// Why a guess is rejected, or the guess itself.
fn check_guess(text: &str) -> Result<[u8; 4], &'static str> {
    if text.chars().count() != 4 {
        return Err("A guess must be exactly 4 digits.");
    }
    let mut code = [0u8; 4];
    for (i, ch) in text.chars().enumerate() {
        match ch.to_digit(10) {
            Some(d) => code[i] = d as u8,
            None => return Err("A guess may contain digits only."),
        }
    }
    if has_duplicate(&code) {
        return Err("Digits may not repeat.");
    }
    Ok(code)
}

pub fn parse_guess(text: &str) -> Option<[u8; 4]> {
    check_guess(text).ok()
}

/// The reason a guess was rejected (empty if it is valid).
fn guess_problem(text: &str) -> &'static str {
    check_guess(text).err().unwrap_or("")
}

/// Returns `None` when the player types `q`.
fn read_guess() -> Option<[u8; 4]> {
    loop {
        print!("Guess (or q to quit): ");
        io::stdout().flush().expect("failed to flush output");
        let text = read_line_trimmed();
        if text.eq_ignore_ascii_case("q") {
            return None;
        }
        match parse_guess(&text) {
            Some(code) => return Some(code),
            None => println!("{}", guess_problem(&text)),
        }
    }
}

pub fn print_code(code: &[u8; 4]) {
    for digit in code {
        print!("{digit}");
    }
    println!();
}

/// Returns (bulls, cows).
pub fn score(secret: &[u8; 4], guess: &[u8; 4]) -> (u8, u8) {
    let mut bulls = 0;
    let mut cows = 0;
    for i in 0..4 {
        if guess[i] == secret[i] {
            bulls += 1;
        } else {
            for j in 0..4 {
                if guess[i] == secret[j] {
                    cows += 1;
                }
            }
        }
    }
    (bulls, cows)
}

pub fn play() {
    let seed = if DEBUG { 42 } else { rand::random::<u64>() };
    let mut rng = StdRng::seed_from_u64(seed);
    let secret = generate_secret(&mut rng);
    if DEBUG {
        print!("(debug) secret: ");
        print_code(&secret);
    }
    println!("I picked a 4-digit code with no repeated digits. You have {MAX_ATTEMPTS} guesses.");
    for attempt in 1..=MAX_ATTEMPTS {
        println!("Guess {attempt} of {MAX_ATTEMPTS}");
        let guess = match read_guess() {
            Some(code) => code,
            None => {
                print!("You quit. The code was ");
                print_code(&secret);
                return;
            }
        };
        let (bulls, cows) = score(&secret, &guess);
        println!("Bulls: {bulls}  Cows: {cows}");
        if bulls == 4 {
            println!("You cracked the code in {attempt} guesses.");
            return;
        }
    }
    print!("Out of guesses. The code was ");
    print_code(&secret);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Supplied by the instructor.
    #[test]
    fn all_bulls() {
        assert_eq!(score(&[1, 2, 3, 4], &[1, 2, 3, 4]), (4, 0));
    }

    #[test]
    fn no_matches() {
        assert_eq!(score(&[1, 2, 3, 4], &[5, 6, 7, 8]), (0, 0));
    }

    #[test]
    fn all_cows() {
        assert_eq!(score(&[1, 2, 3, 4], &[4, 3, 2, 1]), (0, 4));
    }

    #[test]
    fn one_bull_two_cows() {
        assert_eq!(score(&[1, 2, 3, 4], &[1, 3, 5, 2]), (1, 2));
    }

    // Written by the student.
    #[test]
    fn duplicate_detected() {
        assert!(has_duplicate(&[1, 2, 1, 4]));
        assert!(!has_duplicate(&[1, 2, 3, 4]));
    }

    #[test]
    fn parse_rejects_bad_guesses() {
        assert_eq!(parse_guess("1234"), Some([1, 2, 3, 4]));
        assert_eq!(parse_guess("123"), None);
        assert_eq!(parse_guess("12a4"), None);
        assert_eq!(parse_guess("1224"), None);
    }
}
