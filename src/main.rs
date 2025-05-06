mod helper;
use rand::prelude::*;
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

    #[structopt(about = "nested commands", name = "nested")]
    Nested(RandomNumber),
}

#[derive(StructOpt)]
enum RandomNumber {
    #[structopt(about = "print a random number between a range", name = "random")]
    Print(helper::RandomNumberOptions),
}

fn main() -> Result<()> {
    let args = TodoCli::from_args();
    let folder = "./items/";
    match args.cmd {
        TodoCommand::Add(args) => helper::add(folder, &args),
        TodoCommand::List(args) => helper::list(folder, &args),
        TodoCommand::Remove(args) => helper::remove(folder, &args),
        TodoCommand::Nested(args) => {
            match args {
                RandomNumber::Print(args) => {
                    let mut rng = rand::rng();
                    let random_number = rng.random_range(args.low..=args.high);
                    println!(
                        "Random number between {} and {}: {}",
                        args.low, args.high, random_number
                    );
                }
            }
            Ok(())
        }
    }
}

// Follow ups
// 1. Multiple flags together
