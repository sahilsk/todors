use anyhow::{Context, Error as AnyhowError, Result};
use std::{
    env,
    io::{self, Error as stdIoError},
    path::PathBuf,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodorsError {
    #[error("transparent")]
    Io(#[from] stdIoError),

    #[error("file already exist: {0}")]
    FileExist(String),

    #[error("transparent")]
    Unknown(#[from] anyhow::Error),
}

pub fn init(listname: &String) -> Result<(), TodorsError> {
    let mut cfg_dir = create_cfg_dir().context("Failed to create cfg dir")?;

    cfg_dir.push(listname);
    let ownedString = cfg_dir.display().to_string();
    if let Err(e) = std::fs::File::create_new(&cfg_dir.as_path()) {
        match e.kind() {
            std::io::ErrorKind::AlreadyExists => {
                return Err(TodorsError::FileExist(ownedString));
            }
            _ => return Err(TodorsError::Io(e)),
        }
    };

    Ok(())
}

pub fn create_cfg_dir() -> Result<PathBuf> {
    let CFG_DIR_NAME = ".todors";

    let mut cfg_dir = match env::home_dir() {
        Some(home_dir) => home_dir,
        None => {
            return Err(AnyhowError::msg("Screw home dir"));
        }
    };

    cfg_dir.push(CFG_DIR_NAME);
    std::fs::create_dir_all(&cfg_dir)?;

    Ok(cfg_dir)
}
