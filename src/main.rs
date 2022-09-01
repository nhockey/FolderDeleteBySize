#![allow(non_snake_case)]
extern crate fs_extra;
use fs_extra::dir::get_size;
use std::{fs::read_dir, *};

struct Folder {
    name: String,
    size: u64,
}

impl Folder {
    fn new(name: String) -> Folder {
        let size = get_size(name.as_str()).unwrap();
        Folder { name, size }
    }
}

fn main() {
    let mut folders = Vec::new();
    for entry in read_dir(".").unwrap() {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();
        if metadata.is_dir() {
            let folder = Folder::new(entry.file_name().into_string().unwrap());
            folders.push(folder);
        }
    }
    let mut size: u64 = 100;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        size = args[1].parse::<u64>().unwrap();
    }
    println!("This program gives 0 fucks, all folders below {}MB are deleted!", size);
    println!("Are you sure you want to delete all folders smaller than {}MB?", size);
    println!("Type 'yes' to continue");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "yes" {
        println!("Deleting folders under {size}MB...");
        for folder in folders {
            if folder.size < size * 1024 * 1024 {
                println!("Deleting: {}", folder.name);
                fs::remove_dir_all(folder.name.as_str()).unwrap();
            }
        }
    }
}
