extern crate clap;

use std::fs;
use clap::Parser;

#[derive(Parser)]
struct Find {
    //dir to look for
    dir: std::path::PathBuf,
    //path to look within
    path: std::path::PathBuf,
}

fn children(path: &std::path::PathBuf, target: &std::path::PathBuf)  {
   // get all children, iterate over each one and call children function, and print name of dir 
    let dirs = match fs::read_dir(path) {
        Ok(dirs) => dirs,
        Err(_) => return,
   };

    for file in dirs {
        if let Ok(dir) = file {
            let line = dir.path().display().to_string();
            let files: Vec<&str> = line.split("/").collect();
            let file = files.get(files.len() - 1).unwrap();
            //println!("{}", line);
            if file.eq(&target.display().to_string()) {
                println!("{} found in {}", target.display().to_string(), line);
            }
            let lib = "Library";
            if !file.eq(&lib) {
                children(&dir.path(), target);
            }
        }
    }
   
}

fn main() {
    let args = Find::parse();
    children(&args.path, &args.dir);
}
