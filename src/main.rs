use clap::{Parser, Subcommand, Args};
use todors::*;

#[derive(Parser)]
#[command(name = "Todors")]
#[command(version = "1.0")]
#[command(about = "Does awesome things", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    List(ListArgs),
    Task(TaskArgs),
}

#[derive(Args, Debug)]
struct ListArgs {
    #[arg(short, long)]
    new: Option<String>,

    #[arg(short, long)]
    set: Option<String>,
}

#[derive(Args, Debug)]
struct TaskArgs {
    name: Option<String>,
}


fn main() {
    let cli = Cli::parse();
    println!("Hello, world!");

    match &cli.command {
        Commands::List(new) => {
            println!("Creating new list : {new:?}");
        },
        Commands::List(set) => {
            println!("Set context list : {set:?}");
        },
        Commands::Task(name) => {
            println!(" name is: {name:?}" );
        }
    }
}


#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test_cfg_new() {
        let cfg = Config::new("".to_string(), "todors");

    }

}
