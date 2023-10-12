extern crate clap;
extern crate jwalk;

use std::fs;
use std::path::Path;
use std::cmp::max;
use clap::Parser;
use jwalk::WalkDir;

//TODO: 
// add fuzzy finding to the empty function
// publish v0.3.0

#[derive(Parser)]
#[command(version)]
struct Find {
    //find empty dirs 
    #[arg(long, value_name = "EMPTY", help = "find all empty directories in given path")]
    empty: bool,

    //search within hidden files/dirs
    #[arg(long, value_name = "HIDDEN", help = "include hidden directories and files in search")]
    hidden: bool,

    //do a fuzzy find
    #[arg(long, value_name = "FUZZY", help = "do a fuzzy find. the ten closest results will be displayed")]
    fuzzy: bool,

    //look for file extension
    #[arg(short, long, value_name = "EXTENSION", help = "search for all files with a given extension.\nenter file extension without the '.', for example a plain text file as 'txt'")]
    extension: Option<String>,

    //dir to look for
    #[arg(short, long, value_name = "FILE", help = "search for files or directories with given name")]
    file: Option<std::path::PathBuf>,

    //path to look within
    path: std::path::PathBuf,
}

fn lcs(a: &String, b: &&str) -> u32 {
    let a_slice = a.as_bytes();
    let b_slice = b.as_bytes();
    let m = a.len();
    let n = b.len();
    let mut arr = vec![vec![0u32 ; n + 1] ; m + 1];

    let mut i = 1;
    let mut j = 1;
    while i <= m {
        while j <=  n {
            if a_slice[i - 1] == b_slice[j - 1] {
                arr[i][j] = arr[i - 1][j - 1] + 1;
            }
            else {
                arr[i][j] = max(arr[i][j - 1], arr[i - 1][j]);
            }
            j += 1;
        }
        i += 1;
        j = 1;
    }

    arr[m][n]
}

fn children(path: &std::path::PathBuf, target: &std::path::PathBuf, hidden: bool, fuzzy: bool, results: &mut Vec<(u32, usize, String)>)  {
    for file in WalkDir::new(path).skip_hidden(!hidden) {
        match file.as_ref() {
            Ok(string) => {
                let line = &string.path().display().to_string();
                let files: Vec<&str> = line.split("/").collect();
                let f = files.get(files.len() - 1).unwrap();
                if fuzzy {
                    //call fuzzy with target and current filename
                    let lcs = lcs(&target.display().to_string(), f);
                    //add tuple of (lcs, path) to results vec
                    if lcs > 0 {
                        results.push((lcs, f.to_string().len(), line.to_string()));
                    }
                }
                else if f.eq(&target.display().to_string()) {
                    println!("{}", line);
                }
            },
            Err(_) => continue,
        }
    }
}

fn extensions(path: &std::path::PathBuf, ext: &str, hidden: bool) {
    for file in WalkDir::new(path).skip_hidden(!hidden) {
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

fn empty(path: &std::path::PathBuf, hidden: bool, fuzzy: bool) {
    for file in WalkDir::new(path).skip_hidden(!hidden) {
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
    let mut results: Vec<(u32, usize, String)> = Vec::new();
    /*
    if args.empty {
        empty(&args.path, args.hidden, args.fuzzy);
    } 
    */
    //else {
        match args.file {
            //verify that extension is None, if it is also Some then return error
            Some(file) => {
                match args.extension {
                    Some(_) => 
                        panic!("both file and extension provided, please provide one but not both"),
                    None => { 
                        children(&args.path, &file, args.hidden, args.fuzzy, &mut results);
                        if args.fuzzy {
                            //sort results by first param in tuple     
                            results.sort_by_key(|k| (!k.0, k.1));
                            //print top n (10 for now) results
                            let mut i = 0;
                            while i < 10 {
                                println!("{}", results[i].2);
                                i += 1;
                            }
                        }
                    },
                }
            },
            None => {
                match args.extension {
                    Some(ext) => extensions(&args.path, &ext, args.hidden),
                    None => panic!("no args given"),
                }
            },
        }
    //}
}
