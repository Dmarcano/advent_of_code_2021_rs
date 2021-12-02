mod lib;
pub mod days; 

use reqwest::blocking::Client;
use lib::{get_input, SessionID};

fn main() {
    let client = Client::new();
    let session_id = SessionID::new().unwrap();
    get_input(&client, &session_id);
}
