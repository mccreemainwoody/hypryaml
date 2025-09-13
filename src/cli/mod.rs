use clap::Parser;
use std::process::exit;

mod commands;

/// clap command Parser definition
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    command: commands::Commands,
}

/// Main callback of the CLI.
///
/// The right command is run depending of the user's input. Doesn't return
/// anything, but will print an error if application fails.
pub fn run_cli() {
    let cli = CLI::parse();

    match &cli.command {
        commands::Commands::Apply { config } => {
            let result = commands::run_apply(&config);

            match result {
                Ok(_) => {}
                Err(reason) => {
                    println!("Error during theme updating:\n{}", reason);
                    exit(1)
                }
            }
        }
    }
}
