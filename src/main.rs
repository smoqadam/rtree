extern crate clap;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use clap::{Arg, App};

fn main() {
    let args = App::new("rtree")
        .version("0.1")
        .arg(Arg::with_name("path").help("Directory path").required(false).index(1))
        .get_matches();
    let path = args.value_of("path").unwrap_or("./");

    println!("Path: {}", path);
    dir_list(path, 1, 1);
}


fn dir_list(dir: &str, level: usize, parent_level: usize) {
    let paths = fs::read_dir(dir).unwrap();


    let ch = "─";
    if level == 1 {}
    for path in paths {
        let full_path = path.unwrap();

        if get_file_name(&full_path, false).starts_with(".") {
            continue;
        }

        for i in 0..parent_level {
            print!("{:>level$}▕", "", level = (i + 2));
        }
        let filename = get_file_name(&full_path, false);
        if is_dir(&full_path) {
            println!("{ch} {filename}", ch = ch, filename = filename);
            dir_list(&full_path.path().to_str().unwrap(), level + 1, level);
        } else {
            println!("{ch} {d}", ch = ch, d = filename);
        }
    }
}

fn is_dir(path: &DirEntry) -> bool {
    Path::new(path.path().to_str().unwrap()).is_dir()
}

fn get_file_name(path: &DirEntry, full: bool) -> String {
    let p = path.path();
    if full {
        return String::from(path.path().to_str().unwrap());
    }

    let filename = p.file_name().and_then(|n| n.to_str().map(|s| String::from(s))).unwrap();
    String::from(filename)
}