use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    // program description
    println!("{}", "** Guess the number! **".blue());

    // generate secret number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // loop until the user passes
    loop {
        // standard output is buffered, the write is completed but only into memory
        // to print to the console, i must trigger a flush or append a new line character
        // like this
        // print!("Please input your guess: \n");
        // or like this
        print!("Please input your guess: ");
        io::stdout()
            .flush()
            .expect("failed to write a flush for some reason?");

        // accept user's random guess
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // convert the answer into an integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // print the results
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too large!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
