extern crate clap;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use clap::{Arg, App};

fn main() {
    let args = App::new("rtree")
        .version("0.1")
        .arg(Arg::with_name("path").help("Directory path").required(false).index(1))
        // .arg(Arg::with_name("all")
        //          .short("a")
        //          .takes_value(false)
        //          .help("show all files")
        //          .required(false)
        //          .index(2))
        .get_matches();
    let mut path = args.value_of("path").unwrap_or("./");
    // let show_all = args.value_of("all").unwrap_or("false");
    dir_list(path, 0);
}


fn dir_list(dir: &str, level: usize) {
    let paths = fs::read_dir(dir).unwrap();
    let ch = "--";
    for path in paths {
        let dir = path.unwrap();
        if is_dir(&dir) {
            let dirname = get_file_name(&dir, false);
            if dirname.starts_with(".") {
                continue;
            }
            print!("{:>level$}", "", level = level);
            println!("|{ch} {dirname}", ch = ch, dirname = dirname);
            let filename = get_file_name(&dir, true);
            dir_list(&filename, level + 4);
        } else {
            let filename = get_file_name(&dir, false);
            print!("{:>level$}", "", level = level);
            println!("|{ch} {d}", ch = ch, d = filename);
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