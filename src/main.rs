use std::{
    fs::File,
    io::{Error, ErrorKind, Result},
    path::Path,
};
use structopt::StructOpt;

fn main() -> Result<()> {
    let args = TodoCli::from_args();
    let folder = "./items/";
    match args.cmd {
        TodoCommand::Add(args) => add(folder, &args),
    }
}

#[derive(StructOpt)]
struct TodoCli {
    #[structopt(subcommand)]
    cmd: TodoCommand,
}

#[derive(StructOpt)]
enum TodoCommand {
    #[structopt(about = "add an item", name = "add")]
    Add(AddOptions),
}

#[derive(StructOpt)]
struct AddOptions {
    #[structopt(long)]
    item: String,
}

fn add(folder: &str, args: &AddOptions) -> Result<()> {
    let path = build_path_from_item(folder, &args.item);
    if Path::new(&path).exists() {
        Err(Error::new(
            ErrorKind::AlreadyExists,
            "An item with the same name already exists",
        ))
    } else {
        println!("{}", path);
        File::create(path)?;
        Ok(())
    }
}

fn build_path_from_item(folder: &str, item: &str) -> String {
    let folder = folder.replace("/", "\\");
    let path = folder + item;
    println!("Path GP,{}", path);
    path
}
