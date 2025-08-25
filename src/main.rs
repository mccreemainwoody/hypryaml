use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands
}


#[derive(Subcommand)]
enum Commands {
    Apply { config: String }
}


fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Commands::Apply { config } => println!("{}", config),
    }
}
