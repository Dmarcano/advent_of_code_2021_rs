pub mod days;
pub mod libs;
pub mod utils;

use days::Day;
use libs::solve_day;
use std::env;
use std::io;
use std::str::FromStr;
use utils::get_input::get_input;

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

    let input = get_input(day);
    solve_day(day, input)
}
