mod cli;
mod engine;
mod events;
mod json;
mod names;
mod render;

use clap::Parser;

fn main() {
    let args = cli::Cli::parse();
    if let Err(msg) = cli::run(args) {
        eprintln!("hijri: {}", msg);
        std::process::exit(1);
    }
}
