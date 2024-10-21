use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser)]
#[clap(version, about)]
#[clap(propagate_version=true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(about = "add a todo item.")]
    Add {
        #[clap(help="The item content to add.")]
        content: Option<String>,
    },
    #[clap(about = "remove a todo item.")]
    #[clap(visible_aliases = &["rm"])]
    Remove {
        #[clap(help= "The item id to remove")]
        id: Option<String>,
    },
    #[clap(about = "list all todo items.")]
    #[clap(visible_aliases = & ["ls", "ll", "la"])]
    List,
}