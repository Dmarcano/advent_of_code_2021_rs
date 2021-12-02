
use reqwest::{
    blocking::{Client, ClientBuilder, RequestBuilder, Response},
    Url,
};

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

use crate::{days::{get_day_fn, Day}};

use serde_json::Value;
use std::fs::read_to_string;

/// gets an input for a given day 
pub fn get_input(day : Day) -> String { 

    if let Ok(input) = get_input_file(day) { 
        input
    } else { 
        let response = get_input_web(day); 
        let input = response.text().unwrap();
        save_input_file(day, &input);
        input 
    }
}

fn get_input_web(day : Day) -> Response { 
    let client = Client::new();
    let SessionID { id } = SessionID::new().unwrap();

    let url = format!("https://adventofcode.com/2021/{}/input", day.to_string());
    let url = Url::parse(&url).unwrap();
    let request = client
        .get(url)
        .header("cookie", format!("session={}", id))
        .build()
        .unwrap();

    client
        .execute(request)
        .map_err(|e| format!("Error executing request {}", e))
        .unwrap()

}

/// tries to get the input for the given day from the local file system
fn get_input_file(day : Day) -> Result<String,  &'static str> { 
    Err("Not implemented")
}

/// Tries to save an input for the given day to the local file system
fn save_input_file(day : Day, input : &str) -> Result<(), &'static str> { 
    Err("Not implemented")
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
