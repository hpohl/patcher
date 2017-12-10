extern crate serde_json;

use std::env;
use std::collections::{HashMap};
use std::io::{Read};
use std::fs::{File};
use std::path::{Path};

use error::{Error, Result};
use properties::{Properties};


#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq)]
pub enum Platform {
    Web
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Architecture {
    Wasm32
}

#[derive(Debug)]
pub struct Config {
    properties: Properties,
    platforms: HashMap<Platform, Vec<Architecture>>,
}

impl Config {
    pub const VERSION: u64 = 0;

    pub fn parse(string: &str) -> Result<Self> {
        let value: serde_json::Value = try!(serde_json::from_str(string));
        let version = try!(value["version"].as_u64().ok_or("version is not a number"));

        if version != Self::VERSION {
            return Err(Error::from("version mismatch"));
        }

        Ok(Config {
            properties: serde_json::from_value(value["properties"].clone()).unwrap(),
            platforms: serde_json::from_value(value["platforms"].clone()).unwrap(),
        })
    }

    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents));
        Ok(try!(Self::parse(&contents)))
    }

    pub fn read_current() -> Result<Self> {
        let mut cwd = try!(env::current_dir());
        cwd.push("trail.json");
        Self::read(cwd)
    }
}
