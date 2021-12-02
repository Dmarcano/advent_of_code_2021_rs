use crate::days::Day;

use reqwest::{
    blocking::{Client, Request, Response},
    Url,
};
use serde_json::Value;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
pub struct SessionID {
    pub id: String,
}

impl SessionID {
    pub fn new() -> Result<SessionID, &'static str> {
        let token_str = read_to_string("token.json").or(Result::Err(
            "Could not open token.json. Make sure that token.json is in the top level directory",
        ))?;
        let token: Value =
            serde_json::from_str(&token_str).or(Result::Err("Could not parse token.json"))?;

        Ok(SessionID {
            id: token["token"].as_str().unwrap().to_string(),
        })
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_request(&self, day: Day, client: &Client) -> Request {
        let url = Url::parse(&format!("https://adventofcode.com/2021/{}/input", day)).unwrap();

        client
            .get(url)
            .header("cookie", format!("session={}", self.id))
            .build()
            .unwrap()
    }
}

/// gets an input for a given day
pub fn get_input(day: Day) -> String {
    if let Ok(input) = get_input_file(day) {
        input
    } else {
        println!(
            "Could not find input for day {}, Querying the AOC server",
            day
        );
        let response = get_input_web(day);
        let status = response.status();
        let input = response.text().unwrap();

        println!("Got input for day, trying to save to data/input directory");
        let _ = save_input_file(day, &input).map_err(|e_| println!("{}", e_));
        input
    }
}

fn get_input_web(day: Day) -> Response {
    let session = SessionID::new().unwrap();
    let client = Client::new();
    let request = session.get_request(day, &client);

    // let url = format!("https://adventofcode.com/2021/{}/input", day.to_string());
    // let url = Url::parse(&url).unwrap();

    client
        .execute(request)
        .map_err(|e| format!("Error executing request {}", e))
        .unwrap()
}

/// tries to get the input for the given day from the local file system
fn get_input_file(day: Day) -> Result<String, &'static str> {
    let path = format!("data/input/day{}.txt", day.to_u8());
    read_to_string(&path).or(Result::Err("Could not open input file"))
}

/// Tries to save an input for the given day to the local file system
fn save_input_file(day: Day, input: &str) -> Result<(), String> {
    let path = format!("data/input/day{}.txt", day.to_u8());
    println!("Saving input to {}", path);
    std::fs::write(path, input).map_err(|e| format!("Could not write to file: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_id_new() {
        let session_id = SessionID::new();
        println!("{:?}", session_id);
        assert!(session_id.is_ok());
    }
}
