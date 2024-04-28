extern crate walkdir;

use std::path::Path;

use clap::Parser;
use regex::Regex;
use walkdir::WalkDir;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[clap(
    author = "Danny F. Pires",
    version,
    about = "File and Directory Search"
)]
struct Search {
    /// The path to start the search default is "./"
    starting_path: Option<String>,
    #[clap(long)]
    /// The pattern to search for
    name: String,
    #[clap(long = "type")]
    /// The type of search
    search_type: Option<String>,
    #[clap(long = "max-open")]
    /// Max open paths, doesn't impact final results but tradesoff memory for speed
    max_open: Option<usize>,
    #[clap(long = "exclude")]
    /// Paths you which to exclude can be set as a comma separated list
    excluded_paths: Option<String>,
}

fn string_to_regex(search_term: String) -> Regex {
    let regex_search_term: String = if search_term.contains('*') {
        let replaced_search_term = search_term.replace(".", r"\.").replace("*", "(.*)");
        String::from("^") + replaced_search_term.as_str() + &String::from("$")
    } else {
        String::from("^") + search_term.as_str() + &String::from("$")
    };
    let regex = Regex::new(&regex_search_term).unwrap();
    //println!("Regex search term: {}", regex_search_term);
    return regex;
}

fn search_file(file: walkdir::DirEntry, regex: Regex) -> () {
    let file_path: &Path = file.path();
    let file_name: &str = file.file_name().to_str().unwrap_or("");
    if file.file_type().is_file() && regex.is_match(file_name) {
        println!("{}", file_path.display());
    }
}

fn search_dir(file: walkdir::DirEntry, regex: Regex) -> () {
    let file_path: &Path = file.path();
    let file_name: &str = file.file_name().to_str().unwrap_or("");
    if file.file_type().is_dir() && regex.is_match(file_name) {
        println!("{}", file_path.display());
    }
}

fn search_all_types(file: walkdir::DirEntry, regex: Regex) {
    let file_path: &Path = file.path();
    let file_name: &str = file.file_name().to_str().unwrap_or("");
    if regex.is_match(file_name) {
        println!("{}", file_path.display());
    }
}

fn main() {
    let args: Search = Search::parse();
    let starting_dir: String = match args.starting_path {
        Some(x) => x,
        None => String::from("."),
    };
    let search_term: String = args.name; // Bound search by tearm by start and end
    let search_type: String = match args.search_type {
        Some(x) => x,
        None => String::from(""),
    };
    let func: &dyn Fn(walkdir::DirEntry, regex::Regex) -> () = match search_type.as_str() {
        "f" => &search_file,
        "d" => &search_dir,
        _ => &search_all_types,
    };
    let max_open: usize = match args.max_open {
        Some(x) => x,
        None => 1,
    };
    let exclude_string = args.excluded_paths.unwrap_or(String::from(""));
    let exclude_list: Vec<&str> = if exclude_string.is_empty() {
        vec![]
    } else {
        exclude_string.split(",").collect::<Vec<&str>>()
    };
    let regex: Regex = string_to_regex(search_term);
    if exclude_list.len() > 0 {
        for file in WalkDir::new(&starting_dir)
            .max_open(max_open)
                .into_iter()
                .filter_entry(|entry| {
                    for exclude_item in &exclude_list {
                        if entry.path().to_str().unwrap().contains(exclude_item) {
                            return false;
                        }
                    }
                    return true;
                })
        .filter_map(|file| file.ok()) {
            func(file, regex.clone());
        }
    } else {
        for file in WalkDir::new(&starting_dir)
            .max_open(max_open)
                .into_iter()
                .filter_map(|file| file.ok()) {
                    func(file, regex.clone());
                }
    }
}
