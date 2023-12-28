use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::sync::Mutex;

use chrono::{DateTime, Local};
use lazy_static::lazy_static;

lazy_static! {
    static ref GLOBAL_MAP: Mutex<HashMap<String, u8>> = Mutex::new(HashMap::new());
}

#[allow(dead_code)]
fn valid_guess(guess: &str) -> bool {
    guess.len() == 4
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn run_create_file() {
    match File::create("hello.txt") {
        Ok(_val) => println!("File created!"),
        Err(_err) => println!("Error: could not create file."),
    }
}

#[allow(dead_code)]
fn run_format_time() {
    let local: DateTime<Local> = Local::now();
    let time_str = local.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("{}", time_str);
}

#[allow(dead_code)]
fn log_time(filename: &'static str) -> io::Result<()> {
    let local: DateTime<Local> = Local::now();
    let time_str = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    let mut f = File::create(filename)?;
    f.write_all(time_str.as_bytes())?;
    Ok(())
}

#[allow(dead_code)]
fn append_file(filename: &'static str, bytes: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;
    file.write_all(bytes)?;
    Ok(())
}

#[allow(dead_code)]
fn get_formatted_time() -> String {
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    formatted
}

#[allow(dead_code)]
fn run_append_file() {
    match append_file("log.txt", get_formatted_time().as_bytes()) {
        Ok(_) => println!("File created or appended!"),
        Err(_) => println!("Error: could not create file."),
    }
}

fn run_global_mut() {
    // GLOBAL_MAP.insert("one".to_string(), 1);
    let mut global_map = GLOBAL_MAP.lock().unwrap();
    global_map.insert("two".to_string(), 2);
}

fn main() {
    // run_input_length();
    // run_create_file();
    // run_format_time();
    // run_append_file();
    let mut global_map = GLOBAL_MAP.lock().unwrap();
    println!("{:?}", global_map);
    global_map.insert("one".to_string(), 1);
    println!("{:?}", global_map);

    drop(global_map);
    run_global_mut();

    let global_map = GLOBAL_MAP.lock().unwrap();
    println!("{:?}", global_map);
}
