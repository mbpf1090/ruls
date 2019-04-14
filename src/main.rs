extern crate colored;

use std::env;
use std::path;
use std::process;
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
    //let paths = path.read_dir().expect("No such directory");
    let paths = path.read_dir().unwrap_or_else( |err| {
        println!("No such directory: {}", err);
        process::exit(1);
    });

    for item in paths {

        let item = match item {
            Ok(dir) => dir,
            Err(_) => {continue},
        };
        
        if item.metadata().expect("Error getting metadata").is_dir() {
            println!("{} -> {}", mode_to_perm_str(&item.metadata().expect("Error getting permissions").permissions().mode()), item.file_name().into_string().expect("Error getting filename").green());
        } else {
            println!("{} -> {}", mode_to_perm_str(&item.metadata().expect("Error getting permissions").permissions().mode()), item.file_name().into_string().expect("Error getting filename").yellow());
        }
    }
}

fn mode_to_perm_str(entry: &u32) -> String {
    let mut s = String::new();
    let mut mode = *entry;
    mode = mode & 0b0000000_111_111_111;
    for i in 0..3 {
        if i > 0 {
            mode = mode << 3;
        }
        let mode_masked = mode & (0b111 << 6);
        if mode_masked & (1 << 8) == 0 {
            s.push('-')
        } else {
            s.push('r')
        }
        if mode_masked & (1 << 7) == 0 {
            s.push('-')
        } else {
            s.push('w')
        }
        if mode_masked & (1 << 6) == 0 {
            s.push('-')
        } else {
            s.push('x')
        }
        s.push(' ');
    }
    s
}

#[cfg(test)]
mod test {
    use super::mode_to_perm_str;
    #[test]

    fn parse_permission() {
        let mode: u32 = 33188;
        assert_eq!(mode_to_perm_str(&mode), "rw- r-- r-- ");
    }
}