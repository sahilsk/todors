use anyhow::{Context, Result};
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodorsError {
    #[error("IO Error")]
    DiskIo(#[from] anyhow::Error),
}

pub fn init(listname: &String) -> Result<()> {
    /*
     *- create tmp dir
     * - stoe config file
     */

    let CFG_DIR_NAME = ".todors";
    let mut cfg_dir = env::home_dir().expect("Missing home directory");

    //create the config dir and file
    cfg_dir.push(CFG_DIR_NAME);

    std::fs::create_dir_all(&cfg_dir.as_path())
        .with_context(|| format!("Failed to create config dir {cfg_dir:?}"))
        .map_err(TodorsError::DiskIo)?;

    cfgr_dir.push(listname);
    std::fs::File::create_new(&cfgr_dir.as_path())
        .with_context(|| format!("Failed to create list: {&cfgr_dir:?}"))
        .map_err(TodorsError::DiskIo)?;

    Ok(())
}

pub fn create_cfg_dir() -> Result<(), <dyn Error>> {

    let CFG_DIR_NAME = ".todors";

    let mut cfg_dir:std::path::PathBuf = env::home_dir().unwrap_or("~/".into());
    cfg_dir.push(CFG_DIR_NAME);

    std::fs::create_dir_all(cfg_dir)
        .with_context(|| format!("Failed to create cfg dir: {cfg_dir:?}"))
        .map_err(TodorsError::DiskIo)?;

    Ok(());
}
