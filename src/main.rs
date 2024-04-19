use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,

    /// A category for the command
    #[arg(short, long, global = true)]
    category: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Adds a new item
    Add {
        /// The item to add
        item: String,
        /// The amount by which to increase the tally for this item 
        count: u32,
    },
    /// Edits an existing item
    Edit {
        /// The id of the item to edit
        id: u32,
    },
    /// Lists all items
    List,
    /// Removes an item
    Remove {
        /// The id of the item to remove
        id: u32,
    }
}

fn increment(item: String, count: u32) {
    println!("Incrementing {} by {}", item, count);
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
        Command::Edit { id } => println!("Editing item with ID: {}", id),
        Command::List => list(),
        Command::Remove { id } => remove(id),
    }
}
