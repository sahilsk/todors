use clap::{Parser, Subcommand, Args};
use todors::*;
use anyhow::Error;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
struct ListArgs {
    #[arg(short, long)]
    new: Option<String>,

    #[arg(short, long)]
    set: Option<String>,
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
    id_or_partial_desc: String,
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

fn main()  -> Result<(), Error>{
    let cli = Cli::parse();
    println!("Hello, world!");

    match &cli.command {
        Commands::List(args)=> {
            if let Some(newlist) = &args.new {
                println!("Creating new list : {:?}", newlist);
                init(&newlist)?;
            }
            if let Some(setlist) = &args.set{
                println!("Setting list : {:?}", setlist);
            }
        },
        Commands::Init(args) => {
            println!("Init command called");
        }
        Commands::Use(args) => {
            println!("Use command called");
        }
        Commands::Add(args) => {
            println!("Add command called");
        }
        Commands::Done(args) => {
            println!("Done command called");
        }
        Commands::Undone(args) => {
            println!("Undone command called");
        }
        Commands::Rm(args) => {
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

}
