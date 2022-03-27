use std::fs::File;
use std::sync::RwLock;

use roux::Subreddit as RouxSub;
use serde::Deserialize;
use serde::Serialize;

use crate::error::DnrError;
use crate::error::Result;

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
        let serialized = serde_yaml::to_vec(&subreddit)?;
        self.subreddits.push(subreddit);
        let file = self.file.write()?;

        Ok(())
    }
}

pub fn config() -> File {
    let app_dirs = platform_dirs::AppDirs::new(Some(CACHE_DIR_NAME), false)
        .expect("couldn't get app directories");
    let path = app_dirs.cache_dir.join(CONFIG_NAME);

    if let Err(why) = std::fs::create_dir_all(&app_dirs.cache_dir) {
        panic!("couldn't create cache directory: {:?}", why)
    }

    std::fs::File::create(&path).expect("Couldn't open config file")
}
