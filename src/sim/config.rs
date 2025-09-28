use std::fs;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Bodies {
    pub pos: Vec<[f64; 3]>,
    pub vel: Vec<[f64; 3]>,
    pub mass: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub g: f64,
    pub dt: f64,
    pub steps: i64,
    pub softening: f64,
    pub bodies: Bodies,
}

pub fn read_config(path: &str) -> Result<Config> {
    let file = std::fs::read_to_string(path)?;
    let deserialized = serde_json::from_str(&file)?;
    Ok(deserialized)
}

pub fn write_config(path: &str, c: &Config) -> Result<()> {
    let json = serde_json::to_string_pretty(&c)?;
    fs::write(path, json)?;
    Ok(())
}
