mod guessr;
mod executionr;

fn main() {
    test();
}

fn test() {
    
    let mut guesser = guessr::Guessr::guessr("data/English_Word_Prevalences.csv", "data/words.txt", ".....").unwrap();
    let mut executioner = executionr::Executionr::executionr(String::from("APPLE"));
    
    while executioner.wrong_guesses <= 6 && !guesser.already_won() {
        guesser.print_wordlist();
        let guess = guesser.guess();
        let pattern = executioner.execute(guess);
        executioner.print_bad_guesses();
        guesser.new_regex(pattern);
    }
    
}
