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
        // 
        count: u32,
    },
    /// Edits an existing item
    Edit {
        /// The id of the item to edit
        #[arg(short, long)]
        id: u32,
    },
    /// Lists all items
    List,
    /// Removes an item
    Remove,
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Add { item, count } => println!("Adding {} to {}", count, item),
        Command::Edit { id } => println!("Editing item with ID: {}", id),
        Command::List => println!("Listing items"),
        Command::Remove => println!("Removing an item"),
    }
}
