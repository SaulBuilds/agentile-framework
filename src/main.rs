use state_space_music_box::Cli;
use clap::Parser;
use std::process;

fn main() {
    if let Err(e) = Cli::parse().execute() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}