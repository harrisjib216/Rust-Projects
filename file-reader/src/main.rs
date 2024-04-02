use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::{self, read_to_string, stdin, BufReader, Write};

fn main() {
    let content = read_file();

    loop {
        let mut to_find = String::new();
        print_carret();

        match stdin().read_line(&mut to_find) {
            Ok(_) => {
                clear_input();
                search_for_text(&content, &to_find.trim());
            }
            Err(err) => {
                println!("Could not understand\nError: {}", err);
                break;
            }
        }
    }
}

fn read_file() -> String {
    // set cli args
    let args = App::new("file-reader")
        .version("1.0.0")
        .about("Prints the contents of a given file")
        .arg(
            Arg::with_name("filename")
                .help("The name of the file to read")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // get file name
    let filename = args.value_of("filename").unwrap();

    // open and read file
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let content = read_to_string(reader).unwrap();
    println!("```` {}", filename);
    println!("{}", content);
    println!("````");

    // return content
    content
}

fn search_for_text(content: &String, to_find: &str) {
    let re = Regex::new(to_find).unwrap();
    let matches: Vec<_> = re.find_iter(content).map(|m| m.as_str()).collect();

    if matches.is_empty() {
        println!("\x1b[31mNo matches found for:\x1b[0m {}", to_find);
        return;
    }

    if matches.len() == 1 {
        println!("\x1b[32mFound 1 match\x1b[0m");
    } else {
        println!(
            "\x1b[32mFound {} matches for:\x1b[0m {}",
            matches.len(),
            to_find
        );
    }

    for line in matches {
        println!("{}", line);
    }
}

fn print_carret() {
    println!("\nEnter some text to find");
    print!("> ");
    io::stdout().flush().unwrap();
}

fn clear_input() {
    // clear current line
    print!("\x1B[2K\x1B[1G");
    io::stdout().flush().unwrap();

    // move up and clear previous line
    for _ in 0..2 {
        print!("\x1B[F\x1B[2K");
        io::stdout().flush().unwrap();
    }
}
