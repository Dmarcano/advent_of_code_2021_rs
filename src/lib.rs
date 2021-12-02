use serde_json::Value;
use std::fs::read_to_string;
use reqwest::{blocking::{Client, ClientBuilder, RequestBuilder},  Url};


pub fn get_input(client : &Client, SessionID { id }: &SessionID) { 
    // https://adventofcode.com/2021/day/1/input
    let url = Url::parse("https://adventofcode.com/2021/day/1/input").unwrap();
    let request =client.get(url).header("cookie", format!("session={}", id)).build().unwrap();
    println!("{:#?}", request);

    let resp = client.execute(request).unwrap();

    println!("{:#?}", resp.text().unwrap());

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
