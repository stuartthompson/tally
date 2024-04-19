mod tally_args;
mod tally_command;

use clap::Parser;
use tally_args::Args;
use tally_command::Command;

fn increment(item: String, count: u32) {
    println!("Incrementing {} by {}", item, count);
}

fn edit(item_id: u32) {
    println!("Editing item {}", item_id);
}

fn list() {
    println!("Listing current tallies.");
}

fn remove(item_id: u32) {
    println!("Removing item {}", item_id);
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Add { item, count } => increment(item, count),
        Command::Edit { id } => edit(id),
        Command::List => list(),
        Command::Remove { id } => remove(id),
    }
}
