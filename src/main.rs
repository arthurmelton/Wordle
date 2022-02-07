use colored::*;
use std::io::Write;
use std::io::{stdin, stdout};
mod word;

fn main() {
    let word = word::rand_word();
    let mut got_word = false;
    for _ in 0..6 {
        if !got_word {
            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .expect("error: unable to read user input");
            while !word::is_allowed(input.trim().to_string()) || input.trim().len() != 5 {
                println!("{}", "I am sorry but this is not a valid word or it does not have 5 letters, try again".red());
                input = String::new();
                stdin()
                    .read_line(&mut input)
                    .expect("error: unable to read user input");
            }
            let mut t = term::stdout().unwrap();
            let _ = t.cursor_up();
            let _ = t.delete_line();
            for i in 0..5 {
                if input.chars().nth(i).unwrap() == word.chars().nth(i).unwrap() {
                    print!("{}", input.chars().nth(i).unwrap().to_string().green());
                } else if word
                    .chars()
                    .collect::<Vec<char>>()
                    .contains(&input.chars().nth(i).unwrap())
                {
                    print!("{}", input.chars().nth(i).unwrap().to_string().yellow());
                } else {
                    print!("{}", input.chars().nth(i).unwrap().to_string().normal());
                }
            }
            stdout().flush().ok().expect("Could not flush stdout");
            println!("");
            got_word = input.trim() == word;
        }
    }
    if !got_word {
        println!(
            "{} {}",
            "I am sorry that you did not get the word, but the word was",
            word.bold()
        );
    }
}
