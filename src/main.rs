use clap::{Parser,ValueEnum};

// Available commands
#[derive(Debug, Clone, ValueEnum)]
enum Command {
    Add,
    List,
    Edit,
    Remove,
}

// Structure describing the program arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Command to run
    command: Command
}

fn main() {
    let args = Args::parse();

    println!("Command: {:?}", args.command)
}
