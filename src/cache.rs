use std::sync::RwLock;
use std::{fs::File, io::Write};

use roux::Subreddit as RouxSub;
use serde::{Deserialize, Serialize};

use crate::error::{DnrError, Result};

pub const CACHE_DIR_NAME: &str = "dnaaprfr";
pub const FAILED_TO_GET_CACHE_DIR: &str = "couldn't get cache directory";
pub const CONFIG_NAME: &str = "config.yaml";

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Subreddit {
    name: String,
}

impl From<&RouxSub> for Subreddit {
    fn from(o: &RouxSub) -> Subreddit {
        Subreddit {
            name: o.name.clone(),
        }
    }
}

impl From<&Subreddit> for RouxSub {
    fn from(o: &Subreddit) -> RouxSub {
        RouxSub::new(o.name.as_str())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    subreddits: Vec<Subreddit>,
    #[serde(skip)]
    file: RwLock<File>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            subreddits: Vec::default(),
            file: RwLock::new(config()),
        }
    }
}

impl Config {
    pub fn subreddits(&self) -> Vec<RouxSub> {
        self.subreddits.iter().map(|r| r.into()).collect()
    }

    pub fn add_subreddit(&mut self, sub: &RouxSub) -> Result<()> {
        let subreddit = sub.into();
        let serialized = serde_yaml::to_string(&subreddit)?;
        self.subreddits.push(subreddit);
        let mut file = match self.file.write() {
            Ok(file) => file,
            Err(_) => return Err(DnrError::WriteError),
        };

        file.write_all(&serialized.as_bytes())?;

        Ok(())
    }
}

pub fn config() -> File {
    let app_dirs = directories::BaseDirs::new().expect("couldn't get app directories");
    let path = app_dirs.cache_dir().join(CACHE_DIR_NAME).join(CONFIG_NAME);

    if let Err(why) = std::fs::create_dir_all(&app_dirs.cache_dir().join(CACHE_DIR_NAME)) {
        panic!("couldn't create cache directory: {:?}", why)
    }

    std::fs::File::create(&path).expect("couldn't open config file")
}

#[cfg(test)]
mod tests {
    use crate::error::Result;

    #[test]
    fn test_serialize() -> Result<()> {
        Ok(())
    }
}
