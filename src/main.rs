use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Not enough arguments! Usage: $ badgrep 'searchstring' [optional: files]");
        process::exit(1)
    }

    let search = &args[1];
    let mut directory = "./";

    if args.len() >= 3 {
        directory = &args[2];
    }

    println!("Searching {} for '{}'!", directory, search);

    let matches = search_dir_recursive(directory, search);

    if matches.len() == 0 {
        println!("No matches found :-(");
    } else {
        for file in matches {
            println!("{} is a match", file);
        }
    }
}

fn search_dir_recursive(dir_to_search: &str, search: &str) -> Vec<String> {
    let mut matches = vec![];

    let paths = fs::read_dir(dir_to_search).unwrap();

    for path in paths {
        let unwrapped_path = path.unwrap().path();

        let md = fs::metadata(&unwrapped_path).unwrap();

        if md.is_file() {
            let contents = fs::read_to_string(&unwrapped_path);

            let contents = match contents {
                Ok(contents) => contents,
                Err(_error) => continue,
            };

            if contents.contains(search) {
                matches.push(unwrapped_path.to_str().unwrap().to_owned());
            }
        };

        if md.is_dir() {
            matches.append(&mut search_dir_recursive(
                unwrapped_path.to_str().unwrap(),
                search,
            ));
        }
    }

    return matches;
}
