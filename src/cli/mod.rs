use clap::Parser;

mod commands;


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    command: commands::Commands,
}


pub fn run_cli() {
    let cli = CLI::parse();

    match &cli.command {
        commands::Commands::Apply { config } =>
            commands::run_apply(&config)
                .expect("Error during theme updating: ")
    }
}
