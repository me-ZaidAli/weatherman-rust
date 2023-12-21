use clap::Parser;

use weatherman::{run, Arguments};

fn main() {
    let args: Arguments = Arguments::parse();

    if let Err(e) = run(&args) {
        eprintln!("Application error: {}", e)
    }
}
