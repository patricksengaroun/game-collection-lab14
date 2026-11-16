// Shared input helpers.
use std::io::{self, Write};

/// Read one line from standard input and trim it. Ends the program if the input is closed.
pub fn read_line_trimmed() -> String {
    let mut line = String::new();
    let bytes = io::stdin().read_line(&mut line).expect("failed to read input");
    if bytes == 0 {
        println!();
        std::process::exit(0);
    }
    line.trim().to_string()
}

/// Ask for a whole number from `min` to `max` (inclusive). Re-prompts until valid.
pub fn read_int(prompt: &str, min: u32, max: u32) -> u32 {
    loop {
        print!("{prompt}");
        io::stdout().flush().expect("failed to flush output");
        let text = read_line_trimmed();
        match text.parse::<u32>() {
            Ok(n) if n >= min && n <= max => return n,
            Ok(_) => println!("Enter a number from {min} to {max}."),
            Err(_) => println!("That is not a number."),
        }
    }
}
