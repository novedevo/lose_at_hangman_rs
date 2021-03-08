pub struct Executionr {
    pub wrong_guesses: u32,
    guess_limit: u32,
    word: String,
    current_regex: regex::Regex,
}

impl Executionr {
    pub fn executionr(word: String) -> Executionr {
        Executionr {
            wrong_guesses: 0,
            guess_limit: 6,
            current_regex: regex::Regex::new(&".".repeat(word.len())).unwrap(),
            word,
        }
    }
    pub fn execute(&mut self, guess: char) -> regex::Regex {
        if !self.word.contains(guess) {
        self.wrong_guesses += 1;
        } else {
            let word_as_vector: Vec<char> = self.word.chars().collect();

            let next_regex_string = self.current_regex.as_str().chars().enumerate().map(|(index, letter)| {
                if *word_as_vector.get(index).unwrap() == guess {
                    guess
                } else {
                    letter
                }
            });
            
            self.current_regex = regex::Regex::new(&next_regex_string.collect::<String>()).unwrap();
        }
        
        self.current_regex.clone()
        
    }
    pub fn _print_bad_guesses(&self){
        println!("{}", self.wrong_guesses)
    }
}
