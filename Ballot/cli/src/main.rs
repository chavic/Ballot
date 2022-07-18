mod cli;

use clap::Parser;
use cli::*;

fn main() {
    let cli = Cli::parse();
    let debug = cli.debug;
    let command = cli.command;

    let result = {
        if let Some(command) = command {
            match command {
                Commands::Circle { command } => handle_circle_command(command),
                Commands::Ballot { command } => handle_ballot_command(command),
                Commands::Store { command } => handle_store_command(command),
            }
        } else {
            Err(String::from(""))
        }
    };

    println!("{}", result.unwrap())
}
