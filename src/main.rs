pub mod days;

pub mod libs;

use days::Day;

use libs::{get_input, SessionID, solve_day};
use reqwest::blocking::Client;

fn main() {
    let client = Client::new();
    let session_id = SessionID::new().unwrap();
    let day = Day::Day1;

    let response = get_input(&client, &session_id, day);
    let input = response.text().unwrap();
    solve_day(day, input)
}
