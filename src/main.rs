use std::fs;
use clap::Parser;

#[derive(Parser)]
struct Find {
    //dir to look for
    dir: std::path::PathBuf,
    //path to look within
    path: std::path::PathBuf,
}

fn children(path: &std::path::PathBuf)  {
   // get all children, iterate over each one and call children function, and print name of dir 
    let dirs = match fs::read_dir(path) {
        Ok(dirs) => dirs,
        Err(_) => return,
    };

    for file in dirs {
        if let Ok(dir) = file {
            println!("{:?}", dir);
            children(&dir.path());
        }
    }
   
}

fn main() {
    let args = Find::parse();
    children(&args.path);
}
