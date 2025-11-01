use std::env;

use clap::Parser;

/// Binary Manager, it will install or update binary from GitHub, CLI, or Bash Script.
#[derive(Parser, Debug)]
#[command(version, about, long_about =None)]
struct Args {
    /// Configuration file path
    #[arg(short, long)]
    file_path: String,
}


fn main() {
    let args: = Args::parse();
    dbg!(args);

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
