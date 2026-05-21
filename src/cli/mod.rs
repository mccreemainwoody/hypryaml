use clap::Parser;
use std::process::exit;

mod commands;

/// clap command Parser definition
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    command: commands::Commands,

    #[arg(
        long,
        short = 't',
        help = "Duration in milliseconds for which notifications will be displayed",
        default_value_t = 5000
    )]
    notification_timeout: u32,
}

/// Main callback of the CLI.
///
/// The right command is run depending of the user's input. Doesn't return
/// anything, but will print an error if application fails.
pub fn run_cli() {
    let cli = CLI::parse();

    match &cli.command {
        commands::Commands::Apply { config } => {
            let result =
                commands::run_apply(&config, &cli.notification_timeout);

            match result {
                Ok(_) => println!("Configuration succesfully applied!"),
                Err(reason) => {
                    println!("Error during theme updating:\n{}", reason);
                    exit(1)
                }
            }
        }
    }
}
