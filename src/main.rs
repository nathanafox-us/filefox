use std::{env::current_dir, fs::{create_dir, read_dir, remove_dir, rename, DirEntry}, io::Error, path::PathBuf};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Group files with a similar pattern into a new directory
    Group {
        /// Patter to match on
        regex: String,

        /// Name of directory to search through
        #[arg(short, long, default_value = "./")]
        src: Option<PathBuf>,

        /// Optional name of new directory to create (defaults to regex)
        #[arg(short, long, value_name = "DIRECTORY")]
        destination: Option<PathBuf>
    },
    
    /// Extract files from a directory and delete the directory
    Cut {
        /// Directory to remove
        dir: PathBuf,

        /// Name of directory to search through
        #[arg(short, long, default_value = "./")]
        src: PathBuf,
    }
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    if args.command.is_none() {
        println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n~~~ Welcome to filefox!! ~~~\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        println!("\nPlease use the --help option for a list of cmds :)\n");
        return Ok(())
    }

    match &args.command.unwrap() {
        Commands::Group { regex, src, destination } => {
            if !src.as_ref().unwrap().exists() {
                println!("\nError! The directory supplied: {:?} cannot be found!\n", src.as_ref().unwrap());
                return Ok(());
            }

            if destination.is_none() {
                return group(&regex, src.clone().unwrap(), PathBuf::from(&regex));
            } else {
                return group(&regex, src.clone().unwrap(), destination.clone().unwrap());
            }
        },
        Commands::Cut { dir, src } => {
            let mut source = src.clone();
            source.push(dir);
            return cut(&source);
        },
    }      
}

// Group files by REGEX into a file named New_Dir_Name
// filefox group fox -> (group all files whose names include fox in a new directory called fox)
fn group(regex: &str, src: PathBuf, destination: PathBuf) -> Result<(), std::io::Error> {
    println!("\nGrouping files in {:?} by {}", &src, &regex);

    // Get the files to group
    let files: Vec<Result<DirEntry, Error>> = read_dir(&src).unwrap().filter(|file_entry| {
        file_entry.as_ref().unwrap().file_name().into_string().unwrap().to_ascii_lowercase().contains(&regex.to_ascii_lowercase())
    }).collect();

    // Make a new directory
    if files.len() > 0 {
        let path = src.to_str().unwrap().to_string() + "/" + destination.to_str().unwrap();

        let create_result = create_dir(&path);
        if create_result.is_err() {
            println!("Failed to create a directory named {:?}\n", &path);
            return create_result;
        }

        // Move the files into the new directory
        for file in files {
            let new_name = path.clone() + "/" + file.as_ref().unwrap().file_name().to_str().unwrap();
            println!("{:?}", new_name);

            let err = rename(file.as_ref().unwrap().path(), new_name);
            if err.is_err() {
                return err;
            }
        }
        println!("Successfully grouped files from {:?} to {:?}\n", src, path);
        return Ok(());
    } else {
        println!("No files in {:?} matched {:?}", src, regex);
        return Ok(());
    }
}

// If path is to a directory, extract the files from it and delete it!
fn cut(src: &PathBuf) -> Result<(), std::io::Error> {
    println!("\nCutting out the directory {:?}", src);
    if src == &PathBuf::from("./") {
        println!("Error! We won't let you cut the current directory!\n");
        return Ok(());
    }
    
    let files: Vec<Result<DirEntry, Error>> = read_dir(src).unwrap().collect();
    let parent = src.parent().unwrap().to_owned();
    
    let parent_route = if parent.to_str() == Some("") {
        current_dir()
    } else {
        Ok(parent)
    };

    // Move the files into the new directory
    for file in files {
        let new_name = parent_route.as_ref().unwrap().to_str().unwrap().to_owned() + "/" + file.as_ref().unwrap().file_name().to_str().unwrap();
        println!("{:?}", new_name);

        let err = rename(file.as_ref().unwrap().path(), new_name);
        if err.is_err() {
            return err;
        }
    }

    println!("Successfully cut out directory {:?}\n", src);

    // Delete the old directory
    return remove_dir(src);
}