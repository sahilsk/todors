#[allow(unused)]
use clap::{Parser, Subcommand, Args};
use todors::*;
use std::time::SystemTime;
use anyhow::{Error, Result};
use serde_json::*;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ///  Config init
    Config(ConfigArgs),
    ///  New list
    New(NewArgs),
    /// Adds files to myapp
    List(ListArgs),
    /// Init the list
    Init(InitArgs),
    /// Set the list for add/update tasks
    Use(UseArgs),
    /// Add new task to the list
    Add(AddArgs),
    /// Mark task done
    Done(DoneArgs),
    /// Mark task undone
    Undone(UndoneArgs),
    /// Remove task from the list
    Rm(RmArgs),
}


#[derive(Args, Debug)]
struct ConfigArgs {
    init: String,
}

#[derive(Args, Debug)]
struct NewArgs {
    listname: String,
}

#[derive(Args, Debug)]
struct ListArgs {
    #[arg(short, long)]
    new: Option<String>,
}

#[derive(Args,Debug)]
struct InitArgs {
    listname : String,
}

#[derive(Args,Debug)]
struct AddArgs {
    id_or_partial_desc: String,
}

#[derive(Args,Debug)]
struct UseArgs {
    listname: String,
}

#[derive(Args,Debug)]
struct DoneArgs {
    id_or_partial_desc: String,
}

#[derive(Args,Debug)]
struct UndoneArgs {
    id_or_partial_desc: String,
}

#[derive(Args,Debug)]
struct RmArgs {
    id_or_partial_desc: String,
}

fn main()  -> Result<()>{
    let cli = Cli::parse();

    match &cli.command {
        Commands::Config(args)  => {
            println!("args: {}", &args.init);
            create_cfg_file()?;
        },
        Commands::New(args)  => {
            println!("New command | args: {}", &args.listname);
            TaskList::new(&args.listname)?;
        },

        Commands::List(args)=> {
            if let Some(newlist) = &args.new{
                println!("Creating new list : {:?}", newlist);
            }
        },

        Commands::Init(args) => {
            /*
            -- init <mylist>
            Create a new text file in the config dir
            */
            println!("Init command called");
            let a = TaskList {
                id: "myuid".to_string(),
                listname: "mylist".to_string(),
                filepath: "my/file/path".to_string(),
                created_at: SystemTime::now(),
                last_modified: SystemTime::now(),
                tasks: Vec::new(),
            };
            println!("Tasklist: {}", a);
            println!("Tasklist: {:?}", serde_json::to_string(&a));
        }
        Commands::Use(args) => {
            /*
            -- use <mylist>
            set the context to <mylist>
            */
            println!("Use command called | args: {}", &args.listname);
            TaskList::usee(&args.listname)?;
        }
        Commands::Add(args) => {
            /*
            -- add <my task>
            add task to the list set in the context
            */
            println!("Add command called");
        }
        Commands::Done(args) => {
            /*
            -- done <id or task descr>
            mark the task done in the list set in the ctx
            */
            println!("Done command called");
        }
        Commands::Undone(args) => {
            /*
            -- undone <id or task descr>
            mark the task undone in the list set in the ctx
            */
            println!("Undone command called");
        }
        Commands::Rm(args) => {
            /*
            -- rm <id or task description>
            remove the task from the list
            */
            println!("Rm command called");
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
        println!("{:#?}", Cli::command());
    }

    #[test]
    fn test_get_cfg_dir_path() {
        let home_dir = std::env::home_dir().unwrap_or("~?".into());
        assert_eq!(todors::get_cfg_dir_path(), home_dir.join(".todors"));
    }

    #[test]
    fn test_get_cfg_file_path() {
        let home_dir = std::env::home_dir().unwrap_or("~?".into());
        assert_eq!(todors::get_cfg_file_path(), home_dir.join(".todors").join("todors.json"));
    }

}
