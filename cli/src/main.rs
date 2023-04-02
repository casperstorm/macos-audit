use std::path::PathBuf;

use clap::Parser;
use data::application::Application;

/// Simple program to audit a macOS application
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the macOS application.
    #[arg(short, long)]
    path: PathBuf,
}

fn main() {
    let path = Args::parse().path;

    match Application::try_from(path.as_path()) {
        Ok(application) => {
            for (ent, val) in application.entitlements {
                println!("{ent}");
                println!("{val}");
            }
        }
        Err(error) => println!("{error}"),
    }
}
