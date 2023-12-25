use std::io;

fn main() {
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

fn valid_guess(guess: &str) -> bool {
    guess.len() == 4
}
