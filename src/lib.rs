use std::fs::File;
use std::io;
use std::io::Read;
use std::path;
use std::process;

use colored::Colorize;

fn print_error(msg: String) -> () {
    println!("mgrep : {}", msg.red());
    process::exit(0);
}

// Contains reference to two strings in the args
pub struct Query<'a> {
    pub query: &'a String,
    pub file_name: &'a String,
}

impl<'a> Query<'a> {
    pub fn new(args: &'a [String]) -> Query<'a> {
        if args.len() < 3 {
            print_error("pass sufficient number of parameter's".to_string());
            process::exit(0);
        } else {
            Query {
                query: &args[1],
                file_name: &args[2],
            }
        }
    }
}

fn check_file(file_name: &String, path: &path::Path) -> bool {
    let open_res = File::open(path);

    match open_res {
        // Ok(value) Err(error)
        Ok(_) => true,
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => {
                print_error(
                    format!("File {} not found in current directory", file_name).to_string(),
                );
                false
            }
            io::ErrorKind::PermissionDenied => {
                print_error(format!("Access to opening file {} denied", file_name).to_string());
                false
            }
            _ => {
                print_error(
                    format!("Unexpected error when opening the file {}", file_name).to_string(),
                );
                false
            }
        },
    }
}

fn get_content(path: &path::Path) -> String {
    let open_res = File::open(path);

    // No error handling required
    let mut file = open_res.unwrap();
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => content,
        Err(err) => {
            print_error(format!("cannot read file {}", err.kind()));
            "".to_string()
        }
    }
}

pub fn run(query: &Query) {
    println!(
        "Searching for {} in {}",
        format!("{}", query.query).yellow(),
        format!("{}", query.file_name).yellow()
    );

    let path = path::Path::new(query.file_name);
    check_file(query.file_name, &path); // If file doesn't exit- it exits
                                        // so here the retured value must be true
    let content = get_content(&path);
    let vec: Vec<String> = search_query(query.query, &content);

    for line in vec {
        println!("{}", line);
    }
}

fn search_query<'a>(query: &String, contents: &'a String) -> Vec<String> {
    let mut line_res: Vec<String> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            line_res.push(String::from(line));
        }
    }
    line_res
}
