use std::{env };
use thiserror::{Error};
use anyhow::{Context, Result};


#[derive(Error, Debug)]
pub enum TodorsError{

    #[error("IO Error")]
    DiskIo(#[from] anyhow::Error),
}

pub struct Config {
    pub context: String,
    pub storagedir: String,
}


impl Config {
    pub fn new(context: String, storagedir: String) -> Self{
        Self{
            context,
            storagedir,
        }
    }
    pub fn init(&self) -> Result<&Self> {
        /*
         *- create tmp dir
         * - stoe config file
         */

        let CFG_DIR_NAME = ".todors";
        let mut cfg_dir = env::home_dir().expect("Missing home directory");
        //create the config dir and file
        cfg_dir.push(CFG_DIR_NAME);

        std::fs::create_dir_all(cfg_dir.as_path()).with_context(|| format!("Failed to create config dir {cfg_dir:?}") ).map_err(TodorsError::DiskIo)?;

        Ok(&self)
    }
}




