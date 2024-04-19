use clap::Subcommand;

/// Represents all available commands in the application.
///
/// This enum is used to parse user input from the command line into distinct
/// commands that the application can execute. Each variant corresponds to
/// a specific command with its own set of arguments and functionality.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Adds a new item to the application.
    /// 
    /// The `item` parameter is the name of the item to be added. 
    Add {
        /// The item to add
        item: String,
        /// The amount by which to increase the tally for this item 
        count: u32,
    },

    /// Edits an existing item identified by its unique id.
    /// 
    /// The `id` parameter should match the id of the item that needs to be edited. 
    Edit {
        /// The id of the item to edit.
        id: u32,
    },

    /// Lists all items currently being tallied.
    List,

    /// Removes an item from today's tally.
    Remove {
        /// The id of the item to remove.
        id: u32,
    }
}
