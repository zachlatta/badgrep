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

    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let unwrapped_path = path.unwrap().path();

        let md = fs::metadata(&unwrapped_path);

        if md.unwrap().is_file() {
            let contents = fs::read_to_string(&unwrapped_path).unwrap();

            if contents.contains(search) {
                println!("{} contains {}", unwrapped_path.file_name().unwrap().to_str().unwrap(), search);
            }
        }
    }
}
