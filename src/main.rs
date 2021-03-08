mod executionr;
mod guessr;

fn main() {
    test();
}

fn test() {
    let word = "abstractions".to_uppercase();
    let mut guesser = guessr::Guessr::guessr(
        "data/English_Word_Prevalences.csv",
        "data/words.txt",
        &".".repeat(word.len()),
    )
    .unwrap();
    let mut executioner = executionr::Executionr::executionr(String::from(&word));
    while executioner.wrong_guesses <= 6 && !guesser.already_won() {
        let guess = guesser.guess();
        guesser.print_last_guess();
        let pattern = executioner.execute(guess);
        guesser.new_regex(pattern);
    }
    match guesser.already_won() {
        true => println!("Guesser got it! Your word was {}", word),
        false => println!("Guesser failed to guess your word. Consider trying again with a \
            longer, singular word, or contributing to the project by providing a better dataset.")
    }
}
