#![allow(unused)]
use clap::Parser;
use std::fs::{self, create_dir_all, File};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Arguments {
    /// The complete or relative path to the file. If it ends with a / - will create a folder. If
    /// an extension (e.g.: .txt) a file
    #[clap(name = "path/to/file")]
    supplied_path: String,
}

fn main() {
    let args = Arguments::parse();

    let mut supplied_path = args.supplied_path.clone();

    let result = match supplied_path.pop() {
        Some('/') => create_folder(args.supplied_path),
        Some(_) => create_file(args.supplied_path),
        None => print_instructions(),
    };
}

fn print_instructions() -> Result<(), std::io::Error> {
    println!("Error: no path provided. Make sure you supply the path like this: 'n /path/to/file ");
    Ok(())
}

fn create_file(supplied_path: String) -> Result<(), std::io::Error> {
    let mut path_to_file = PathBuf::new();
    path_to_file.push(&supplied_path);

    match path_to_file.parent() {
        Some(_) => {
            let parent_dirs = path_to_file.parent().unwrap();
            let file_name = path_to_file.file_name().unwrap();

            let absolute_path = Path::new(parent_dirs).join(file_name);

            create_dir_all(parent_dirs);
            let file = File::create(&absolute_path)?;

            println!("Created a file at: {}", absolute_path.canonicalize().unwrap().display());
        }
        None => {
            let file = File::create(&path_to_file)?;
            println!("Created a file at: {}", path_to_file.display());
        }
    }

    Ok(())
}

fn create_folder(supplied_path: String) -> Result<(), std::io::Error> {
    let mut path_to_folder = PathBuf::new();
    path_to_folder.push(supplied_path);

    let parent_dirs = path_to_folder.parent().unwrap();
    let folder_name = path_to_folder.file_name().unwrap();

    let absolute_path = Path::new(parent_dirs).join(folder_name);
    create_dir_all(&absolute_path)?;

    println!("Created a folder at: {}", absolute_path.canonicalize().unwrap().display());

    Ok(())
}
