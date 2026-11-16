// Mad Libs and the number inspector.
use crate::input::read_line_trimmed;
use std::io::{self, Write};
use std::str::FromStr;

fn ask(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("failed to flush output");
    read_line_trimmed()
}

fn ask_number<T: FromStr>(prompt: &str) -> T {
    loop {
        match ask(prompt).parse::<T>() {
            Ok(value) => return value,
            Err(_) => println!("That is not a valid number. Try again."),
        }
    }
}

fn mad_libs() {
    println!("--- Mad Libs ---");
    let name = ask("A name: ");
    let noun = ask("A noun: ");
    let adjective = ask("An adjective: ");
    let verb = ask("A verb: ");
    let place = ask("A place: ");
    let whole: i32 = ask_number("A whole number: ");
    let decimal: f64 = ask_number("A decimal number: ");

    let doubled = whole * 2;
    let halved = decimal / 2.0;
    let summary = format!("{name} took {whole} {noun}s to {place}.");

    println!();
    println!("{summary}");
    println!("It was a very {adjective} day, and {name} decided to {verb}.");
    println!("Twice {whole} is {doubled}, and half of {decimal:.2} is {halved:.2}.");
    println!("Nobody in {place} ever forgot the {adjective} {noun}.");
}

fn inspector() {
    println!("--- Number Inspector ---");
    let number: u32 = ask_number("A whole number (0 or more): ");
    println!("decimal: {number}");
    println!("binary: {number:b}");
    println!("padded binary: {number:08b}");
    println!("hex: {number:x}");
    let text = ask("A single character: ");
    match text.chars().next() {
        Some(c) => println!("'{c}' has the code {}", c as u32),
        None => println!("No character entered."),
    }
}

pub fn play() {
    mad_libs();
    println!();
    inspector();
}
