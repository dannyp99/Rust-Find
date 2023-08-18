use std;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("{}", args.len());
        println!("Invalid number of args!");
        println!("Two args required <starting-dir> <search-term>");
        return;
    }
    let starting_dir: String = args.get(1).unwrap().to_string();
    let search_term: String = args.get(2).unwrap().to_string();
    let temp: &String = &String::from("");
    let search_type: String = args.get(3).unwrap_or_else(|| temp).to_string();
    if search_type != "dir" || search_type != "file" {
        println!("Invalid search type, please provide arguments \"dir\" or \"file\"");
        return;
    }
    search(starting_dir, search_term, search_type);
}

fn search(starting_dir: String, search_term: String, search_type: String) -> () {
    
}

fn get_children_dirs() -> Vec<String> {
 return Vec::new();
}
