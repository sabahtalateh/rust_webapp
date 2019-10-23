use failure::Fallible;
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub log: LogConfig,
    pub postgres: PostgresConfig,
}

impl Config {
    pub fn from_file(filename: &str) -> Fallible<Self> {
        Ok(toml::from_str(&read_to_string(filename)?)?)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub url: String,
    pub cert: PathBuf,
    pub key: PathBuf,
    pub redirect_from: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LogConfig {
    pub actix_web: String,
    pub webapp: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub database: String,
}
