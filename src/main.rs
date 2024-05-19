extern crate walkdir;

use clap::Parser;
use wildmatch::WildMatch;
use walkdir::{DirEntry, WalkDir};

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
    /// Max open paths, doesn't impact final results but tradesoff memory for speed default = 3
    max_open: Option<usize>,
    #[clap(long = "exclude")]
    /// Paths you which to exclude can be set as a comma separated list
    excluded_paths: Option<String>,
}

fn search_file(file: &DirEntry, wildcard: &WildMatch) -> () {
    if file.file_type().is_file() && wildcard.matches(file.file_name().to_str().unwrap_or("")) {
        println!("{}", file.path().display());
    }
}

fn search_dir(file: &DirEntry, wildcard: &WildMatch) -> () {
    if file.file_type().is_dir() && wildcard.matches(file.file_name().to_str().unwrap_or("")) {
        println!("{}", file.path().display());
    }
}

fn search_all_types(file: &DirEntry, wildcard: &WildMatch) {
    if wildcard.matches(file.file_name().to_str().unwrap_or("")) {
        println!("{}", file.path().display());
    }
}

fn main() {
    let args: Search = Search::parse();
    let starting_dir: &str = &args.starting_path.unwrap_or(".".to_string());
    let search_term: &str = &args.name; // Bound search by tearm by start and end
    let search_type: &str = &args.search_type.unwrap_or("".to_string());
    let func: &dyn Fn(&DirEntry, &wildmatch::WildMatch) -> () = match search_type {
        "f" => &search_file,
        "d" => &search_dir,
        _ => &search_all_types,
    };
    let max_open: usize = match args.max_open {
        Some(x) => x,
        None => 3,
    };
    let exclude_string: &str = &args.excluded_paths.unwrap_or("".to_string());
    let wildcard: WildMatch = WildMatch::new(search_term);
    if exclude_string.is_empty() {
        for file in WalkDir::new(&starting_dir)
            .max_open(max_open)
            .into_iter()
            .filter_map(|file| file.ok())
        {
            func(&file, &wildcard);
        }
    } else {
        let exclude_list: Vec<&str> = exclude_string.split(",").collect::<Vec<&str>>();
        for file in WalkDir::new(&starting_dir)
            .max_open(max_open)
            .into_iter()
            .filter_entry(|entry| {
                for exclude_item in &exclude_list {
                    if entry.path().starts_with(exclude_item) {
                        return false;
                    }
                }
                return true;
            })
        {
            if file.is_ok() {
                func(&file.unwrap(), &wildcard);
            }
        }
    }
}
