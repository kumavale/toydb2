extern crate toydb2;

use clap::{Parser, Subcommand};

use toydb2::repl;
use toydb2::server;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// What mode to run the program in
    #[command(subcommand)]
    mode: Option<Mode>,
}

#[derive(Subcommand)]
enum Mode {
    /// Run REPL
    Repl,
    /// Run server
    Server,
}

fn main() {
    let cli = Cli::parse();
    match cli.mode {
        None | Some(Mode::Repl) => {
            repl::run();
        }
        Some(Mode::Server) => {
            server::run();
        }
    }
}
