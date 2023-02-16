use std::path::PathBuf;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the macOS application.
    #[arg(short, long)]
    path: PathBuf,

    /// Show entitlements.
    #[arg(short, long)]
    entitlements: bool,
}

fn main() {
    let args = Args::parse();

    println!("Hello {:?}!", args.path);
    println!("real path? {}", args.path.exists());
    println!("we entitlements? {}", args.entitlements);
}

