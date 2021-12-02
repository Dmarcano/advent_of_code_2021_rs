use reqwest::{
    blocking::{Client, ClientBuilder, RequestBuilder, Response},
    Url,
};

use crate::days::{Day, get_day_fn}; 

use serde_json::Value;
use std::fs::read_to_string;

pub fn solve_day(day : Day, input : String) { 
    let (part_one, part_two) = get_day_fn(day); 
    let result_one = part_one(input);
    let result_two = part_two(result_one.clone());
    println!("Day {} part one: {}", day, result_one);
    println!("Day {} part two: {}", day, result_two);
}

pub fn get_input(client: &Client, SessionID { id }: &SessionID, day : Day ) -> Response {
    // https://adventofcode.com/2021/day/1/input
    let url =format!("https://adventofcode.com/2021/{}/input", day.to_string());
    let url = Url::parse(&url).unwrap();
    let request = client
        .get(url)
        .header("cookie", format!("session={}", id))
        .build()
        .unwrap();
    println!("{:#?}", request);

    client
        .execute(request)
        .map_err(|e| format!("Error executing request {}", e))
        .unwrap()
}

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
