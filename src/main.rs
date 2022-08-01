use std::env;
use std::process;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Not enough arguments! Usage: $ badgrep 'searchstring' [optional: files]");
        process::exit(1)
    }

    let search = &args[1];

    search_dir_recursive("./", search)
}

fn search_dir_recursive(dir_to_search: &str, search: &str) {
    let paths = fs::read_dir(dir_to_search).unwrap();

    for path in paths {
        let unwrapped_path = path.unwrap().path();

        let md = fs::metadata(&unwrapped_path).unwrap();

        if md.is_file() {
            let contents = fs::read_to_string(&unwrapped_path);

            let contents = match contents {
                Ok(contents) => contents,
                Err(_error) => continue
            };

            if contents.contains(search) {
                println!("{} contains {}", unwrapped_path.to_str().unwrap(), search);
            }
        }

        if md.is_dir() {
            search_dir_recursive(unwrapped_path.to_str().unwrap(), search)
        }
    }
}