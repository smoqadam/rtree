use std::fs;
use std::path::Path;

fn main() {
    dir_list("./", 0);
}


fn dir_list(dir: &str, level: i32) {
    let paths = fs::read_dir(dir).unwrap();
    let first = "";
    for path in paths {
        let p = path.unwrap().path();
        if Path::new(p.to_str().unwrap()).is_dir() {
            let d = p.to_str().unwrap();
            println!("{:>level$}{dir}", level = level as usize, dir = d);
            dir_list(d, level + 4);
        } else {
            println!("    {}", p.display());
        }
    }

}