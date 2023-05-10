use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about= None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hello { name: Option<String> },
    Hi { name: Option<String> },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Hello { name } => {
            println!("Hello, Clap!");
            if let Some(value) = name {
                println!("Hello, {}!", value);
            }
        }
        Commands::Hi { name } => {
            println!("Hi, Clap!");
            if let Some(value) = name {
                println!("Hi, {}!", value);
            }
        }
    }
}
