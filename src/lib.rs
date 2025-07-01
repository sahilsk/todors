#[allow(unused)]
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{OpenOptions};
use std::io::{Read, Write};
use std::{io::Error as stdIoError, path::PathBuf, time::SystemTime, time::UNIX_EPOCH};
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

    #[error("transparent")]
    TimeError(#[from] std::time::SystemTimeError),
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
        writeln!(f, "id: {}\n", self.id)?;
        writeln!(f, "name: {}\n", self.listname)?;
        writeln!(f, "path:{}\n", self.filepath)?;
        writeln!(
            f,
            "created_at: {:?}\n",
            self.created_at.duration_since(UNIX_EPOCH)
        )?;
        writeln!(
            f,
            "modified_at: {:?}\n",
            self.last_modified.duration_since(UNIX_EPOCH)
        )?;
        writeln!(f, "----\n\n")?;

        for task in &self.tasks {
            writeln!(
                f,
                "{} | {} | {} | \n",
                task.id, task.status, task.description
            )?;
        }

        writeln!(f, "---")?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    created_at: SystemTime,
    last_modified: SystemTime,
    context: Option<String>,
    tasklist: Vec<String>,
}

impl TaskList {
    pub fn new(listname: &String) -> Result<Self, TodorsError> {
        let cfg_dir = create_cfg_dir().context("Failed to create cfg dir")?;
        let tasklist_file = cfg_dir.join(listname);

        match std::fs::File::create_new(&tasklist_file) {
            Err(e) => match e.kind() {
                std::io::ErrorKind::AlreadyExists => {
                    let task_list =
                        Err(TodorsError::FileExist(tasklist_file.display().to_string()));
                    task_list
                }
                _ => Err(TodorsError::Io(e)),
            },
            Ok(mut file) => {
                let meta = file.metadata().map_err(TodorsError::Io)?;
                let result = TaskList {
                    id: listname.to_string(),
                    listname: listname.to_string(),
                    filepath: tasklist_file.display().to_string(),
                    created_at: std::time::SystemTime::now(),
                    last_modified: meta.modified()?,
                    tasks: Vec::new(),
                };

                let result_str = serde_json::to_string_pretty(&result)?;
                file.write_all(result_str.as_bytes())?;
                println!("written to mylist");
                register_with_config(listname)?;
                println!("printing big json: {:?}", result_str);
                Ok(result)
            }
        }
    }

    pub fn usee(listname: &String) -> Result<()> {
        let cfg_file_path = get_cfg_file_path();
        assert!(std::fs::exists(&cfg_file_path).unwrap_or(false));

        let mut cfg_file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&cfg_file_path)?;

        let mut cfg_content = String::new();
        cfg_file.read_to_string(&mut cfg_content)?;

        let err_msg = format!("{} : not a valid json", CFG_FILE);
        let mut cfg: Config = serde_json::from_str(&cfg_content).expect(&err_msg);
        cfg.context = Some(listname.to_string());
        cfg.last_modified = SystemTime::now();
        println!("cfg : {:?}", cfg);

        cfg_file.set_len(0)?;
        cfg_file.write_all(serde_json::to_string_pretty(&cfg)?.as_bytes())?;

        Ok(())
    }
}

pub fn register_with_config(newlist: &String) -> Result<()> {
    assert!(std::fs::exists(get_cfg_file_path()).unwrap_or(false));
    let mut cfg_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(get_cfg_file_path())?;

    let mut cfg_content = String::new();
    cfg_file.read_to_string(&mut cfg_content)?;

    let mut cfg: Config = serde_json::from_str(&cfg_content)?;
    if !cfg.tasklist.contains(newlist) {
        cfg.tasklist.push(newlist.to_string());
    }
    let cfg_content = serde_json::to_string_pretty(&cfg)?;
    cfg_file.set_len(0)?;
    cfg_file.write_all(cfg_content.as_bytes())?;

    Ok(())
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
    create_cfg_dir()?;
    let cfg_file_path = get_cfg_file_path();

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(cfg_file_path)
        .map_err(TodorsError::Io)?;

    let current_now = SystemTime::now();
    let config = Config{
        created_at: current_now,
        last_modified: current_now,
        context: None,
        tasklist: vec![],
    };
    let content = serde_json::to_string_pretty(&config)?;
    file.write_all(content.as_bytes())?;

    Ok(file)
}
