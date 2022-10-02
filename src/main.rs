#![allow(unused)]
use clap::Parser;
use std::fs::{self, create_dir_all, File};
use std::path::{Path, PathBuf};

/// A very simple utility to create files and folders in your terminal.
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(
    arg_required_else_help(true),
    after_help = "Example commands: \n
    n file.txt \n
    n folder/ \n
    n ../new_folder/file.txt"
)]
struct Arguments {
    /// Complete or relative path to the file (or multiple files). If it ends with a / - the command will create a folder.
    #[clap(name = "path/to/file")]
    supplied_path: Vec<String>,
}

fn main() {
    let args = {
        match Arguments::try_parse() {
            Ok(arguments) => arguments,
            Err(err) => err.exit(),
        }
    };

    for supplied_path in &args.supplied_path {
        match supplied_path.chars().last() {
            Some('/') => create_folder(supplied_path),
            Some(_) => create_file(supplied_path),
            None => print_instructions(),
        };
    }
}

fn print_instructions() -> Result<(), std::io::Error> {
    eprintln!(
        "Error: no path provided. Make sure you supply the path like this: 'n /path/to/file "
    );
    Ok(())
}

fn create_file(supplied_path: &String) -> Result<(), std::io::Error> {
    let mut path_to_file = PathBuf::new();
    path_to_file.push(supplied_path);

    match path_to_file.parent() {
        Some(_) => {
            let parent_dirs = path_to_file.parent().unwrap();
            let file_name = path_to_file.file_name().unwrap();

            let absolute_path = Path::new(parent_dirs).join(file_name);

            create_dir_all(parent_dirs);
            let file = File::create(&absolute_path)?;

            println!(
                "Created a file: {}",
                absolute_path.canonicalize().unwrap().display()
            );
        }
        None => {
            let file = File::create(&path_to_file)?;
            println!("Created a file: {}", path_to_file.display());
        }
    }

    Ok(())
}

fn create_folder(supplied_path: &String) -> Result<(), std::io::Error> {
    let mut path_to_folder = PathBuf::new();
    path_to_folder.push(supplied_path);

    let parent_dirs = path_to_folder.parent().unwrap();
    let folder_name = path_to_folder.file_name().unwrap();

    let absolute_path = Path::new(parent_dirs).join(folder_name);
    create_dir_all(&absolute_path)?;

    println!(
        "Created a folder: {}",
        absolute_path.canonicalize().unwrap().display()
    );

    Ok(())
}
