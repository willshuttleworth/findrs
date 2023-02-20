extern crate clap;
extern crate jwalk;

use std::fs;
use std::path::Path;
use clap::Parser;
use jwalk::WalkDir;

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
    for file in WalkDir::new(path) {
        match file.as_ref() {
            Ok(string) => {
                let line = &string.path().display().to_string();
                let files: Vec<&str> = line.split("/").collect();
                let f = files.get(files.len() - 1).unwrap();
                if f.eq(&target.display().to_string()) {
                    println!("{}", line);
                }
            },
            Err(_) => continue,
        }
    }
}

fn extensions(path: &std::path::PathBuf, ext: &str) {
    for file in WalkDir::new(path) {
        match file.as_ref() {
            Ok(string) => {
                let line = &string.path().display().to_string();
                let files: Vec<&str> = line.split("/").collect();
                let f = files.get(files.len() - 1).unwrap();
                let extension = &f.split('.').nth(1);
                match extension {
                    Some(extension) => {
                        if extension.eq(&ext) {
                            println!("{}", line);
                        }
                    },
                    None => continue,
                }
            },
            Err(_) => continue,
        }
    }
}

fn empty(path: &std::path::PathBuf) {
    for file in WalkDir::new(path) {
        match file.as_ref() {
            Ok(string) => {
                let line = &string.path().display().to_string();
                if Path::new(&line).is_dir() {
                    match fs::read_dir(&file.unwrap().path()) {
                        Ok(dirs) => {
                            if dirs.count() == 0 {
                                println!("{}", line);
                            }
                        },
                        Err(_) => continue,
                    }
                }
            },
            Err(_) => continue,
        };
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
