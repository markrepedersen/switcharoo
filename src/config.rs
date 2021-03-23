use std::{
    fs::File,
    io::{Error, Read},
};

use crate::routes::features::KV;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u64,
    pub features: Vec<KV>,
}

impl Config {
    pub fn parse() -> Result<Self, Error> {
        let mut file = File::open("./config.toml")?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;

        Ok(toml::from_str(&content)?)
    }
}
