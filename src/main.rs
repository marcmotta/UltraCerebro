// src/main.rs
/*
 * Main executable for UltraCerebro
 */

use clap::Parser;
use ultracerebro::{Result, run};

#[derive(Parser)]
#[command(version, about = "UltraCerebro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
