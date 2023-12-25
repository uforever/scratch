use std::fs::{File, OpenOptions};
use std::io::{self, Write};

use chrono::{DateTime, Local};

fn valid_guess(guess: &str) -> bool {
    guess.len() == 4
}

fn run_input_length() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // 97 98 99 100 13 10
    // a  b  c  d  CR LF
    for c in guess.chars() {
        print!("{} ", c as u8);
    }
    println!();
    // 97 98 99 100 10
    // a  b  c  d  LF
    for c in "abcd\n".chars() {
        print!("{} ", c as u8);
    }
    println!();
    println!("{}", guess.len());
    guess = guess.trim().to_string();
    println!("{}", guess.len());
    println!("You guessed: {}", guess);
    println!("is valid: {}", valid_guess(&guess));
    let _greeting = guess.contains("hello world");
}

fn run_create_file() {
    match File::create("hello.txt") {
        Ok(_val) => println!("File created!"),
        Err(_err) => println!("Error: could not create file."),
    }
}

fn run_format_time() {
    let local: DateTime<Local> = Local::now();
    let time_str = local.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("{}", time_str);
}

fn log_time(filename: &'static str) -> io::Result<()> {
    let local: DateTime<Local> = Local::now();
    let time_str = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    let mut f = File::create(filename)?;
    f.write_all(time_str.as_bytes())?;
    Ok(())
}

fn append_file(filename: &'static str, bytes: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;
    file.write_all(bytes)?;
    Ok(())
}

fn get_formatted_time() -> String {
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    formatted
}

fn run_append_file() {
    match append_file("log.txt", get_formatted_time().as_bytes()) {
        Ok(_) => println!("File created or appended!"),
        Err(_) => println!("Error: could not create file."),
    }
}

fn main() {
    // run_input_length();
    // run_create_file();
    // run_format_time();
    run_append_file();
}
