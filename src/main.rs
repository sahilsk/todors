use clap::{Parser, Subcommand, Args};
use todors::*;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Ls(ListArgs),
    Init(InitArgs),
    Use(UseArgs),
    Add(AddArgs),
    Done(DoneArgs),
    Undone(UndoneArgs),
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

fn main() {
    let cli = Cli::parse();
    println!("Hello, world!");

    match &cli.command {
        Commands::Ls(args)=> {
            if let Some(newlist) = &args.new {
                println!("Creating new list : {:?}", newlist);
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
