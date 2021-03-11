use std::{env, io, process::exit};

use regex::Regex;

mod executionr;
mod guessr;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(s) => match s.as_str() {
            "--interactive" => interact(),
            "--help" => help(),
            s => test(s.to_uppercase()),
        },
        _ => {
            eprintln!("Please supply arguments: try --help for more information.");
            eprintln!("If you're using cargo, `cargo run -- --help' is how you would accomplish this.");
            exit(-1);
        }
    };
}

fn help() {
    println!("Guessing game v 0.1.0 help");
    println!("Use command line flag --help for this dialog [e.g. cargo run -- --help]");
    println!("Use --interactive to actually play hangman against the bot [e.g. cargo run -- --interactive]");
    println!(
        "Test the engine by including your desired word as the first command line argument [e.g. cargo run pineapple]"
    );
    exit(0);
}

fn interact() {
    let mut length = String::new();
    let wrong_guesses = 0;
    println!("Please enter the length of your word in letters, e.g. pineapple is 9 letters long");
    io::stdin().read_line(&mut length).expect("Failed to read line");

    let length: usize = length
        .trim()
        .parse()
        .expect("Could not parse your input. Are you sure you entered the right length?");

    let mut guesser = guessr::Guessr::new(
        "data/English_Word_Prevalences.csv",
        "data/words.txt",
        &".".repeat(length),
    )
    .unwrap();

    println!("Please enter your strings with '.' reflecting an unguessed position, e.g. pineapple would be .........");
    println!("After the engine guesses E, you would update your string to be ...E....E");

    while wrong_guesses <= 6 && !guesser.already_won() {
        println!(
            "The engine has guessed the letter {}. Please update your string to reflect this:",
            guesser.guess()
        );
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let processed_string = guess.trim().to_uppercase();
        let new_regex = Regex::new(&processed_string).unwrap();
        guesser.new_regex(new_regex);
    }
    match guesser.already_won() {
        true => println!("Guesser got it! Your word was {}", guesser.final_answer()),
        false => println!(
            "Guesser failed to guess your word. Consider trying again with a \
            longer, singular word, or contributing to the project by providing a better dataset."
        ),
    }
}

fn test(word: String) {
    let mut guesser = guessr::Guessr::new(
        "data/English_Word_Prevalences.csv",
        "data/words.txt",
        &".".repeat(word.len()),
    )
    .unwrap();
    let mut executioner = executionr::Executionr::new(String::from(&word));
    while executioner.wrong_guesses <= 6 && !guesser.already_won() {
        let guess = guesser.guess();
        guesser.print_last_guess();
        guesser.new_regex(executioner.execute(guess));
    }
    match guesser.already_won() {
        true => println!("Guesser got it! Your word was {}", word),
        false => println!(
            "Guesser failed to guess your word. Consider trying again with a \
            longer, singular word, or contributing to the project by providing a better dataset."
        ),
    }
}
