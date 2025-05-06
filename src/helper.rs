use std::{
    fs::{File, read_dir, remove_file},
    io::{Error, ErrorKind, Result},
    path::Path,
};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct AddOptions {
    #[structopt(long)]
    item: String,
}

#[derive(StructOpt)]
pub struct ListOptions {
    #[structopt(long)]
    sorted: bool,
}
#[derive(StructOpt)]
pub struct RemoveOptions {
    #[structopt(long)]
    item: String,
}

#[derive(StructOpt)]
pub struct RandomNumberOptions {
    #[structopt(long)]
    low: i32,
    #[structopt(long)]
    high: i32,
}

impl RandomNumberOptions {
    pub fn get_low(&self) -> i32 {
        self.low
    }

    pub fn get_high(&self) -> i32 {
        self.high
    }
}

pub fn add(folder: &str, args: &AddOptions) -> Result<()> {
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

pub fn list(folder: &str, args: &ListOptions) -> Result<()> {
    let mut paths: Vec<_> = read_dir(folder)?.filter_map(Result::ok).collect();
    if args.sorted {
        paths.sort_by_key(|dir| dir.file_name());
    }
    for path in paths {
        if let Ok(filename) = path.path().strip_prefix(folder) {
            println!("{}", filename.display());
        }
    }
    Ok(())
}

pub fn remove(folder: &str, args: &RemoveOptions) -> Result<()> {
    let path = build_path_from_item(folder, &args.item);
    if !Path::new(&path).exists() {
        Err(Error::new(
            ErrorKind::NotFound,
            "An item with the name does not exists",
        ))
    } else {
        remove_file(path)?;
        Ok(())
    }
}

fn build_path_from_item(folder: &str, item: &str) -> String {
    folder.to_string() + item
}
