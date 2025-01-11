mod commands;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use commands::{random, token_stat};

#[derive(Subcommand)]
enum Commands {
    /// Token statistics for a file
    TokenStat {
        /// Path to a yamd file
        path: PathBuf,
    },
    /// Generate random tokens
    Random {
        /// length of a sequence in bytes
        length: usize,
        /// literal length
        #[clap(default_value = "10")]
        #[arg(short, long)]
        max_literal_len: usize,
    },
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Random {
            length,
            max_literal_len,
        } => random(length, max_literal_len),
        Commands::TokenStat { path } => token_stat(path),
    }
}
