#[allow(unused)]
use anyhow::{Context, Result};
use serde::Serialize;
use serde_json::to_string as json_dumps;
use std::fmt;
use std::io::Write;
use std::{io::Error as stdIoError, path::PathBuf, time::SystemTime};
use thiserror::Error;

const CFG_DIR_NAME: &str = ".todors";
const CFG_FILE: &str = "todors.json";

#[derive(Debug, Error)]
pub enum TodorsError {
    #[error("I/O error: {0}")]
    Io(#[from] stdIoError),

    #[error("file already exist: {0}")]
    FileExist(String),

    #[error("Failed to parse json: {0}")]
    JsonParseFault(#[from] serde_json::Error),

    #[error("transparent")]
    Unknown(#[from] anyhow::Error),
}

#[derive(Serialize, Debug)]
pub struct Task {
    pub id: String,
    pub status: String,
    pub description: String,
    pub added_on: SystemTime,
    pub modified_on: SystemTime,
}

#[derive(Serialize, Debug)]
pub struct TaskList {
    pub id: String,
    pub listname: String,
    pub filepath: String,
    pub created_at: SystemTime,
    pub last_modified: SystemTime,
    pub tasks: Vec<Task>,
}

impl fmt::Display for TaskList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {}\n", self.id);
        write!(f, "name: {}\n", self.listname);
        write!(f, "path:{}\n", self.filepath);
        write!(
            f,
            "created_at: {:?}\n",
            self.created_at.duration_since(SystemTime::UNIX_EPOCH)
        );
        write!(
            f,
            "modified_at: {:?}\n",
            self.last_modified.duration_since(SystemTime::UNIX_EPOCH)
        );
        write!(f, "----\n\n");

        for task in &self.tasks {
            write!(
                f,
                "{} | {} | {} | \n",
                task.id, task.status, task.description
            );
        }

        write!(f, "---");
        Ok(())
    }
}

#[derive(Serialize, Debug)]
pub struct Config {
    created_at: SystemTime,
    last_modified: SystemTime,
    context: Option<String>,
}

impl TaskList {
    pub fn new(listname: &String) -> Result<TaskList, TodorsError> {
        /*
         * Create new list and set it for the context
         */
        let cfg_dir = create_cfg_dir().context("Failed to create cfg dir")?;

        let tasklist_file = cfg_dir.join(listname);
        if let Err(e) = std::fs::File::create_new(&tasklist_file) {
            match e.kind() {
                std::io::ErrorKind::AlreadyExists => {
                    return Err(TodorsError::FileExist(tasklist_file.display().to_string()));
                }
                _ => return Err(TodorsError::Io(e)),
            }
        };

        let meta = std::fs::metadata(&tasklist_file)?;
        let result = TaskList {
            id: listname.to_string(),
            listname: listname.to_string(),
            filepath: tasklist_file.display().to_string(),
            created_at: std::time::SystemTime::now(),
            last_modified: meta.modified()?,
            tasks: Vec::new(),
        };

        let result_str = json_dumps(&result)?;
        println!("printing big json: {:?}", result_str);

        Ok(result)
    }
}

pub fn get_cfg_dir_path() -> PathBuf {
    let home_dir: std::path::PathBuf = std::env::home_dir().unwrap_or("~/".into());
    home_dir.join(CFG_DIR_NAME)
}

pub fn get_cfg_file_path() -> PathBuf {
    let home_dir: std::path::PathBuf = std::env::home_dir().unwrap_or("~/".into());
    home_dir.join(CFG_DIR_NAME).join(CFG_FILE)
}

pub fn create_cfg_dir() -> Result<PathBuf, TodorsError> {
    let cfg_dir = get_cfg_dir_path();
    std::fs::create_dir_all(&cfg_dir).map_err(|e| TodorsError::Io(e))?;

    Ok(cfg_dir)
}

pub fn create_cfg_file() -> Result<std::fs::File, TodorsError> {
    let cfg_dir = create_cfg_dir()?;
    let cfg_file_path = get_cfg_file_path();

    let is_exist: bool = !std::fs::exists(&cfg_file_path).unwrap_or(false);
    if !is_exist {
        let mut file = std::fs::File::create_new(cfg_file_path).map_err(TodorsError::Io)?;

        let config = Config {
            created_at: SystemTime::now(),
            last_modified: SystemTime::now(),
            context: None,
        };

        let content = json_dumps(&config)?;
        std::fs::File::write(&mut file, content.as_bytes());

        return Ok(file);
    }

    Err(TodorsError::Io(std::io::Error::new(
        std::io::ErrorKind::AlreadyExists,
        format!("file already exist {:?}", &cfg_file_path),
    )))
}
