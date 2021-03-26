use std::{
    fs::OpenOptions,
    io::{Error, Read},
};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u64,
}

impl Config {
    pub fn parse() -> Result<Self, Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .create(false)
            .open("./config.toml")?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;

        Ok(toml::from_str(&content)?)
    }
}
