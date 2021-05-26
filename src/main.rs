use std::io;

mod executionr;
mod guessr;

use clap::Clap;

/// A command line interface to help you lose at the wordplay game, hangman.
/// Run with no arguments for the interactive game.
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Devon Burnham <novedevo@gmail.com>")]
struct Opts {
    /// Provide a test word, to see if the engine can solve it!
    #[clap(short, long)]
    test: Option<String>
}

fn main() {
    let opts = Opts::parse();
    match opts.test {
        Some(word) => test(word.to_ascii_uppercase()),
        None => interact(),
    }
}

fn interact() {
    println!("Let's play hangman! You pick a word, and I'll try to guess it.");
    let mut length = String::new();
    let wrong_guesses = 0;
    println!("Please enter the length of your word in letters, e.g. pineapple is 9 letters long");
    io::stdin().read_line(&mut length).expect("Failed to read line");

    let length: usize = length
        .trim()
        .parse()
        .expect("Could not parse your input. Are you sure you entered the right length?");

    let mut guesser = guessr::Guessr::new(&".".repeat(length));

    println!("Please enter your strings with '.' reflecting an unguessed position, e.g. pineapple would be .........");
    println!("After the engine guesses E, you would update your string to be ...E....E");

    while wrong_guesses <= 6 && !guesser.already_won() {
        println!(
            "The engine has guessed the letter {}. Please update your string to reflect this:",
            guesser.guess()
        );
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        guesser.new_regex(&guess.trim().to_uppercase());
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
    let mut guesser = guessr::Guessr::new(&".".repeat(word.len()));
    let mut executioner = executionr::Executionr::new(String::from(&word));
    while !guesser.already_won() && !executioner.already_lost() {
        let guess = guesser.guess();
        guesser.print_last_guess();
        guesser.new_regex(executioner.execute(guess).as_str());
    }
    match guesser.already_won() {
        true => println!("Guesser got it! Your word was {}", word),
        false => println!(
            "Guesser failed to guess your word. Consider trying again with a \
            longer, singular word, or contributing to the project by providing a better dataset."
        ),
    }
}
