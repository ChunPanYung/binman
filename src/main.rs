use clap::Parser;

/// Binary Manager, it will install or update binary from GitHub, CLI, or Bash Script.
#[derive(Parser)]
#[command(name = "binman")]
#[command(version = "0.1")]
#[command(about = "Install or update binary from GitHub, CLI, or Bash Script.")]
struct Cli {
    /// Configuration file path
    #[arg(short, long, value_name = "FILE")]
    file: String,
}


fn main() {
    let cli = Cli::parse();
    // dbg!(args);

    println!("{}", cli.file);
}
