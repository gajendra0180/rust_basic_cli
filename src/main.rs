mod helper;
use std::io::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
struct TodoCli {
    #[structopt(subcommand)]
    cmd: TodoCommand,
}

#[derive(StructOpt)]
enum TodoCommand {
    #[structopt(about = "add an item", name = "add")]
    Add(helper::AddOptions),

    #[structopt(about = "list all items", name = "list")]
    List(helper::ListOptions),

    #[structopt(about = "remove an item", name = "remove")]
    Remove(helper::RemoveOptions),
}

fn main() -> Result<()> {
    let args = TodoCli::from_args();
    let folder = "./items/";
    match args.cmd {
        TodoCommand::Add(args) => helper::add(folder, &args),
        TodoCommand::List(args) => helper::list(folder, &args),
        TodoCommand::Remove(args) => helper::remove(folder, &args),
    }
}

// Follow ups
// 2. add suubcommand inside subcommand and then options
