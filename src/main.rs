pub mod days;

pub mod libs;

use days::Day;
use std::env;
use std::io;
use std::str::FromStr;
use libs::{get_input, solve_day, SessionID};
use reqwest::blocking::Client;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut day_arg = String::new();

    if args.len() >= 2 {
        day_arg = args[1].clone();
    } else {
        println!("Enter day: ");
        io::stdin()
            .read_line(&mut day_arg)
            .expect("Failed to read line");
    }
    day_arg = day_arg.trim().to_string();

    let day = Day::from_str(&day_arg).unwrap();


    let client = Client::new();
    let session_id = SessionID::new().unwrap();

    let response = get_input(&client, &session_id, day);
    let input = response.text().unwrap();
    solve_day(day, input)
}
