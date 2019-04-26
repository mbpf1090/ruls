extern crate atty;
extern crate colored;

use atty::Stream;
use colored::*;
use std::env;
use std::os::unix::fs::PermissionsExt;
use std::path;
use std::process;
use std::fmt;

#[derive(Debug)]
struct Dir {
    name: String,
    permissions: String,
    size: String,
    is_dir: bool,
}

impl Dir {
    fn new(entry: std::fs::DirEntry) -> Dir {
        Dir {
            name: entry.file_name().into_string().expect("Eror getting file name"),
            permissions: mode_to_perm_str(&entry.metadata().expect("Error getting permissions").permissions().mode()),
            size: entry.metadata().expect("Error getting size").len().to_string(),
            is_dir: entry.metadata().expect("Error getting metadata").is_dir()
        }
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_dir {
        write!(f, "{:<15} | {:<10}", self.permissions, self.name)
        } else {
        write!(f, "{: <15} | {: <30} | {: <5} bytes", self.permissions, self.name, self.size)
        }
    }
}

fn main() {
    let mut args = env::args();
    let mut is_tty = true;
    if !atty::is(Stream::Stdout) {
        is_tty = false;
    }
    args.next();

    let path_str: String = match args.next() {
        Some(path) => path,
        None => String::from("."),
    };

    let path: &path::Path = path::Path::new(&path_str);
    println!("{}", path.display());
    let paths = path.read_dir().unwrap_or_else(|err| {
        println!("No such directory: {}", err);
        process::exit(1);
    });

    // Header
    println!("{:<15} | {:<30} | {:<5}", "Permissions", "Name", "Size");
    
    // Table
    for item in paths {
        let item = match item {
            Ok(dir) => Dir::new(dir),
            Err(_) => continue,
        };
        if is_tty {
            // Todo: Add colors
            println!("{}", item);
        } else {
            println!("{}", item);
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
