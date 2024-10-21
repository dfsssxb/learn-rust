use::std::process;

mod cli;
mod database;
mod command;
use clap::Parser;
use cli::{Cli, Commands};
use database::DataBase;

// todo: bug: 删除后，再添加，依然会有问题。文件末尾是最后一行，这一行是空行，则添加的是新行，如果这一行不是空行，则再行末继续添加，导致没有成功换行
fn main() {
    let args = Cli::parse();
    let mut db = DataBase::open("db");
    let result: Result<_, &'static str> = match args.command {
        Commands::Add { content } => command::add(&mut db, content),
        Commands::List => command::list(&mut db),
        Commands::Remove { id } => command::remove(&mut db, id),
    };

    if let Err( err) = result {
        eprint!("error: {}", err);
        process::exit(1);
    }
}
