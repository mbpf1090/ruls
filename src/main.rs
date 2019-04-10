use std::fs;
use std::env;
use std::path;
use std::fs::DirEntry;

fn main() {
    let mut args = env::args();
    args.next();

    let path_str = match args.next() { //String
        Some(path) => path,
        None => String::from("Error")
    };
    
    let path = path::Path::new(&path_str); // Path
    println!("{}", path.display());
    let result = path.read_dir().unwrap();           //Iterator
    for item in result {
        //println!("{}", item.unwrap().path().to_str().unwrap());
        println!("{:?}", item.unwrap().path());
    }
}
