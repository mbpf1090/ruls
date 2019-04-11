use std::env;
use std::path;
use std::fs::{DirEntry, ReadDir};

fn main() {
    let mut args = env::args();
    args.next();

    let path_str: String = match args.next() {
        Some(path) => path,
        None => String::from("Error")
    };
    
    let path: &path::Path = path::Path::new(&path_str);
    println!("{}", path.display());
    let result = path.read_dir().expect("Error getting directory");
    for item in result {
        let item = item.unwrap();
        
        if item.metadata().unwrap().is_dir() {
            println!("{}", item.file_name().into_string().unwrap());
        } else {
            println!("{:?}", item.file_type());
        }
    }
}

