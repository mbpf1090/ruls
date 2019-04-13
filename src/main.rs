extern crate colored;

use std::env;
use std::path;
use std::fs::{DirEntry, ReadDir};
use colored::*;
use std::os::unix::fs::PermissionsExt;

fn main() {
    let mut args = env::args();
    args.next();

    let path_str: String = match args.next() {
        Some(path) => path,
        None => String::from(".")
    };
    
    let path: &path::Path = path::Path::new(&path_str);
    println!("{}", path.display());
    let paths = path.read_dir().expect("Error getting directory");
    for item in paths {
        let item = item.unwrap();
        
        if item.metadata().unwrap().is_dir() {
            println!("{} -> {}",mode_to_perm_str(&item), item.file_name().into_string().unwrap().green());
        } else {
            println!("{} -> {}",mode_to_perm_str(&item), item.file_name().into_string().unwrap().yellow());
        }
    }
}

fn mode_to_perm_str(entry: &DirEntry) -> String {
    let mut s = String::new();
    let mut mode = entry.metadata().unwrap().permissions().mode();
    mode = mode & 0b0000000_111_111_111;
    for i in 0..3 {
        if i > 0 {
            mode = mode << 3;
        }
        let mode_masked = mode & 0b111000000;
        if mode_masked & 0b100000000 == 0 {
            s.push('-')
        } else {
            s.push('r')
        }
        if mode_masked & 0b010000000 == 0 {
            s.push('-')
        } else {
            s.push('w')
        }
        if mode_masked & 0b001000000 == 0 {
            s.push('-')
        } else {
            s.push('x')
        }
        s.push(' ');
    }
    s
}