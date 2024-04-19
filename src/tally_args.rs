use clap::Parser;
use crate::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,

    /// A category for the command
    #[arg(short, long, global = true)]
    category: Option<String>,
}