#[macro_use]
extern crate nickel;

use std::fs::OpenOptions;
use std::io::{Result, Write};

use chrono::{DateTime, Local};
use clap::{Arg, Command};
use nickel::Nickel;

fn formatted_time_entry() -> String {
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    formatted
}

fn record_entry_in_log(filename: String, bytes: &[u8]) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(filename)?;
    file.write_all(bytes)?;
    Ok(())
}

fn log_time(filename: String) -> Result<()> {
    let entry = formatted_time_entry();
    let bytes = entry.as_bytes();

    record_entry_in_log(filename, bytes)?;
    Ok(())
}

fn do_log_time(logfile_path: String) -> String {
    match log_time(logfile_path) {
        Ok(_) => "File created!".to_string(),
        Err(e) => format!("Error: {}", e),
    }
}

fn main() {
    let matches = Command::new("My Program")
        .author("Me, me@mail.com")
        .version("1.0.2")
        .about("Explains in brief what the program does")
        .arg(
            Arg::new("LOG FILE")
                .short('l')
                .long("logfile")
                .required(true)
                .help("The log file to use"),
        )
        .arg(Arg::new("AUTH TOKEN").short('t').long("token"))
        .get_matches();

    let logfile_path = matches.get_one::<String>("LOG FILE").unwrap().to_string();
    let auth_token = matches
        .get_one::<String>("AUTH TOKEN")
        .map(|token| token.to_string());
    println!("{:?}", auth_token);

    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            do_log_time(logfile_path.clone())
        }
    });

    let _ = server.listen("127.0.0.1:6767");
}
