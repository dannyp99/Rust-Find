extern crate walkdir;

use std::{collections::HashSet, path::Path};

use clap::Parser;
use regex::Regex;
use walkdir::WalkDir;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[clap(author = "Danny F. Pires", version, about = "File and Directory Search")]
struct Search {
    /// The path to start the search default is .
    starting_path: Option<String>,
    #[clap(long)]
    /// The pattern to search for
    name: String,
    #[clap(long = "type")]
    /// The type of search
    search_type: Option<String>,
    #[clap(long = "path-only", default_value_t = true)]
    // Print path only
    path_only: bool
}

fn string_to_regex(search_term: String) -> Regex {
    let regex_search_term: String = if search_term.contains('*') {
        let replaced_search_term = search_term.replace("*", "(.*)");
        String::from("^") + replaced_search_term.as_str()
    } else {
        String::from("^") + search_term.as_str() + &String::from("$")
    };
    let regex = Regex::new(&regex_search_term).unwrap();
    return regex;
}


fn search_file(starting_dir: String, regex: Regex) -> () {
        for file in WalkDir::new(starting_dir)
            .max_open(10)
                .into_iter()
                .filter_map(|file| file.ok().filter(|f| f.metadata().unwrap().is_file()))
                {
                    let file_path: &Path = file.path();
                    let file_name: &str = file.file_name().to_str().unwrap_or("");
                    if regex.is_match(file_name) {
                        println!("{}", file_path.display());
                    }
                }
}

fn search_dir(starting_dir: String, regex: Regex) -> () {
    for file in WalkDir::new(starting_dir)
        .max_open(10)
            .into_iter()
            .filter_map(|file| file.ok().filter(|f| f.metadata().unwrap().is_dir()))
            {
                let file_path: &Path = file.path();
                let file_name: &str = file.file_name().to_str().unwrap_or("");
                if regex.is_match(file_name) {
                    println!("{}", file_path.display());
                }
            }
}

fn search_all_types(starting_dir: String, regex: Regex, path_only: bool) {
    for file in WalkDir::new(starting_dir)
        .max_open(10)
            .into_iter()
            .filter_map(|file| file.ok())
    {
        let file_path: &Path = file.path();
        let file_name: &str = file.file_name().to_str().unwrap_or("");
        let match_str: String = if path_only {
            file_path.display().to_string()
        } else if file_path.is_file() {
            format!("Found matching File: {}", file_path.display())
        } else {
            format!("Found matching Directory: {}", file_path.display())
        };
        if regex.is_match(file_name) {
            println!("{}", match_str);
        }
    }
}

fn main() {
    let args: Search = Search::parse();
    let starting_dir: String = match args.starting_path {
        Some(x) => x,
        None => String::from(".")
    };
    let search_term: String = args.name; // Bound search by tearm by start and end
    let search_type: String = match args.search_type {
        Some(x) => x,
        None => String::from(""),
    };
    let regex: Regex = string_to_regex(search_term);
    let valid_types: HashSet<String> =
        HashSet::from([String::from("d"), String::from("f"), String::from("")]);
    if !valid_types.contains(&search_type) {
        println!("-type: {}: unkown type", search_type);
        return;
    }

    match (starting_dir, regex, search_type.as_str()) {
        (x, y, "f") => search_file(x,y),
        (x, y, "d") => search_dir(x,y),
        (x, y, _) => search_all_types(x, y, args.path_only),
    }
}
