use std::{env::current_dir, fs::{create_dir, read_dir, remove_dir, rename, DirEntry}, io::Error, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(value_parser(["group", "cut"]))]
    cmd: Option<String>,

    #[arg(default_value = "/.")]
    src: PathBuf,

    params: Vec<String>,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    if args.cmd == Option::None {
        println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n~~~ Welcome to filefox!! ~~~\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        println!("\nThe format for using this cli tool is:\n");
        println!("filefox <cmd> <src>* <further params>*");
        println!("\nPlease use the --help option for a list of cmds :)\n");
        return Ok(())
    }

    match args.cmd.unwrap().as_str() {
        "group" => {
            if args.params.len() != 1 {
                println!("Error, the group cmd requires exactly one additional parmater: the regex to match on.");
                return Ok(());
            }
            return group(&args.src, &args.params[0]);
        },
        "cut" => {
            return cut(&args.src);
        },
        _ => {
            println!("ERROR, THAT IS NOT A CMD!!");
            return Ok(());
        } 
    }      
}

// Group files by REGEX into a file named New_Dir_Name
// filefox group fox -> (group all files whose names include fox in a new directory called fox)
fn group(src: &PathBuf, regex: &str) -> Result<(), std::io::Error> {
    println!("Grouping files in {:?} by {}", src, regex);

    // Get the files to group
    let files: Vec<Result<DirEntry, Error>> = read_dir(src).unwrap().filter(|file_entry| {
        file_entry.as_ref().unwrap().file_name().into_string().unwrap().to_ascii_lowercase().contains(&regex.to_ascii_lowercase())
    }).collect();

    // Make a new directory
    if files.len() > 0 {
        let path = src.to_str().unwrap().to_string() + "/" + regex;

        let create_result = create_dir(&path);
        if create_result.is_err() {
            println!("Failed to create a directory named {:?}", &path);
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
        println!("Successfully grouped files from {:?} to {:?}", src, path);
        return Ok(());
    } else {
        println!("No files in {:?} matched {:?}", src, regex);
        return Ok(());
    }
}

// If path is to a directory, extract the files from it and delete it!
fn cut(src: &PathBuf) -> Result<(), std::io::Error> {
    println!("Cutting out the directory {:?}", src);
    if src == &PathBuf::from("./") {
        println!("Error! We won't let you cut the current directory!");
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

    // Delete the old directory
    return remove_dir(src);
}