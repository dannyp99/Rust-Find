extern crate walkdir;

use walkdir::WalkDir;
use regex::Regex;
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[clap(author="Danny Pires", version, about="File and Directory Search")]
struct Search {
    /// The path to start the search default is .
    starting_path: String,
    #[clap(long)]
    /// The pattern to search for
    name: String,
    #[clap(long="type")]
    /// The type of search
    search_type: Option<String>
}

fn main() {
    let args: Search = Search::parse();
    let starting_dir: String = args.starting_path; //match args.starting_path {
    //     Some(value) => value,
    //     None => String::from(".")
    // };
    let search_term: String = args.name; // Bound search by tearm by start and end
    let search_type: String  = match args.search_type {
        Some(value) => value,
        None => String::from("")
    };
    //TODO: Add pattern matching based on Some()/None to call different versions of search
    search(starting_dir, search_term, search_type);
}

fn search(starting_dir: String, search_term: String, search_type: String) -> () {
    let empty_str: &String = &String::from("");
    // Enforce ending if there is no wildcard match
    let regex_search_term =  if search_term.chars().nth_back(0).eq(&Option::Some('*')) 
        { String::from("^") + search_term.as_str() }
        else { String::from("^") + search_term.as_str() + &String::from("$")};
    let regex = Regex::new(&regex_search_term).unwrap();
    dbg!(search_type == "d" || search_type == "dir");
    dbg!(search_type == "f" || search_type == "file");
    
    for file in WalkDir::new(starting_dir).max_open(4).into_iter().filter_map(|file| file.ok()) {
        let file_path = file.path();
        let file_name = file.file_name().to_str().unwrap_or_else(|| empty_str);
        if (search_type == "file" || search_type == "f") && file_path.is_file() && regex.is_match(file_name) {
            println!("Found matching File: {}", file_path.display());
        } else if (search_type == "dir" || search_type == "d")
            && file_path.is_dir() && file_path.to_str().unwrap().ends_with(&search_term) {
                println!("Found matching directory: {}", file_path.display());
        }
    }
}

