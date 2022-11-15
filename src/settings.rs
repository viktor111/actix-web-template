use std::error::Error;

use serde::Deserialize;

#[derive(Debug, Deserialize)]

pub struct Settings {
    pub ip: String,
    pub port: u16,
}

impl Settings {
    pub fn new() -> Result<Settings, Box<dyn Error>> {
        let file = std::fs::File::open("src/settings.json");
        match file {
            Ok(file) => {
                let settings = serde_json::from_reader(file).unwrap();
                Ok(settings)
            }
            Err(e) => {
                Err(Box::new(e))
            }
        }
    }
}
