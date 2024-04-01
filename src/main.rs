extern crate walkdir;

use std::{collections::HashSet, path::Path};

use clap::Parser;
use regex::Regex;
use walkdir::{IntoIter, WalkDir};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[clap(author = "Danny F. Pires", version, about = "File and Directory Search")]
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
    max_open: Option<usize>
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


fn search_file(walkdir_iter: IntoIter, regex: Regex) -> () {
    for file in walkdir_iter.filter_map(|file| file.ok())
    {
        let file_path: &Path = file.path();
        let file_name: &str = file.file_name().to_str().unwrap_or("");
        if file.file_type().is_file() && regex.is_match(file_name) {
            println!("{}", file_path.display());
        }
    }
}

fn search_dir(walkdir_iter: IntoIter, regex: Regex) -> () {
    for file in walkdir_iter.filter_map(|file| file.ok()) {
        let file_path: &Path = file.path();
        let file_name: &str = file.file_name().to_str().unwrap_or("");
        if file.file_type().is_dir() && regex.is_match(file_name) {
            println!("{}", file_path.display());
        }
    }
}

fn search_all_types(walkdir_iter: IntoIter, regex: Regex) {
    for file in walkdir_iter.filter_map(|file| file.ok()) {
        let file_path: &Path = file.path();
        let file_name: &str = file.file_name().to_str().unwrap_or("");
        if regex.is_match(file_name) {
            println!("{}", file_path.display());
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
        None    => String::from("")
    };
    let max_open: usize = match args.max_open {
        Some(x) => x,
        None => 1
    };
    let regex: Regex = string_to_regex(search_term);
    let valid_types: HashSet<String> =
        HashSet::from([String::from("d"), String::from("f"), String::from("")]);
    if !valid_types.contains(&search_type) {
        println!("-type: {}: unkown type", search_type);
        return;
    }

    let walkdir_iter: IntoIter = WalkDir::new(starting_dir).max_open(max_open).into_iter();

    match search_type.as_str() {
        "f" => search_file(walkdir_iter,regex),
        "d" => search_dir(walkdir_iter,regex),
        _ => search_all_types(walkdir_iter, regex),
    }
}
