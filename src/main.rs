extern crate clap;

use std::fs;
use std::path::Path;
use clap::Parser;

#[derive(Parser)]
#[command(version)]
struct Find {
    //find empty dirs 
    #[arg(long, value_name = "EMPTY", help = "find all empty directories in given path")]
    empty: bool,

    //look for file extension
    #[arg(short, long, value_name = "EXTENSION", help = "search for all files with a given extension.\nenter file extension without the '.', for example a plain text file as 'txt'")]
    extension: Option<String>,

    //dir to look for
    #[arg(short, long, value_name = "FILE", help = "search for files or directories with given name")]
    file: Option<std::path::PathBuf>,

    //path to look within
    path: std::path::PathBuf,
}

fn children(path: &std::path::PathBuf, target: &std::path::PathBuf)  {
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
            //ignore these macOS directories because they are too big and probably have no user
            //created files
            if !file.eq(&"Library") && !file.eq(&"System") && !file.eq(&"Volumes"){
                children(&dir.path(), target);
            }
        }
    }
}

fn extensions(path: &std::path::PathBuf, ext: &str) {
    let dirs = match fs::read_dir(path) {
        Ok(dirs) => dirs,
        Err(_) => return,
   };

    for file in dirs {
        if let Ok(dir) = file {
            let line = dir.path().display().to_string();
            let files: Vec<&str> = line.split("/").collect();
            let file = files.get(files.len() - 1).unwrap();
            let extension = &file.split('.').nth(1);
            match extension {
                Some(extension) => {
                    if extension.eq(&ext) {
                        println!("{} found in {}", file, line);
                    }
                },
                None => (),
            }
            let lib = "Library";
            if !file.eq(&lib) {
                extensions(&dir.path(), ext);
            }
        }
    }
}

fn empty(path: &std::path::PathBuf) {
    let dirs = match fs::read_dir(path) {
        Ok(dirs) => dirs,
        Err(_) => return,
   };

    for file in dirs {
        if let Ok(dir) = file {
            let line = dir.path().display().to_string();
            let files: Vec<&str> = line.split("/").collect();
            let file = files.get(files.len() - 1).unwrap();
            if Path::new(&line).is_dir() {
                let subdirs = match fs::read_dir(&dir.path()) {
                    Ok(dirs) => dirs,
                    Err(_) => return,
                };
                if subdirs.count() == 0 {
                    println!("empty dir found at {}", line);
                }
            }
            //ignore these macOS directories because they are too big and probably have no user
            //created files
            if !file.eq(&"Library") && !file.eq(&"System") && !file.eq(&"Volumes"){
                empty(&dir.path());
            }
        }
    }
}

fn main() {
    let args = Find::parse();

    if args.empty {
        empty(&args.path);
    } else {
        match args.file {
            //verify that extension is None, if it is also Some then return error
            Some(file) => {
                match args.extension {
                    Some(_) => 
                        panic!("both file and extension provided, please provide one but not both"),
                    None => children(&args.path, &file),
                }
            },
            None => {
                match args.extension {
                    Some(ext) => extensions(&args.path, &ext),
                    None => panic!("no args given"),
                }
            },
        }
    }
}
