pub struct Executionr {
    wrong_guesses: u32,
    guess_limit: u32,
    word: String,
    current_regex: regex::Regex,
}

impl Executionr {
    pub fn new(word: String) -> Self {
        Self {
            wrong_guesses: 0,
            guess_limit: 6,
            current_regex: regex::Regex::new(
                &std::str::from_utf8(&vec![b'.'; word.len()]).unwrap(),
            )
            .unwrap(),
            word,
        }
    }
    pub fn execute(&mut self, guess: u8) -> regex::Regex {
        if !self.word.contains(guess as char) {
            self.wrong_guesses += 1;
        } else {
            let word_as_vector: Vec<u8> = self.word.bytes().collect();

            let next_regex_string = self
                .current_regex
                .as_str()
                .bytes()
                .enumerate()
                .map(|(index, letter)| {
                    if *word_as_vector.get(index).unwrap() == guess {
                        guess
                    } else {
                        letter
                    }
                })
                .collect::<Vec<u8>>();

            self.current_regex =
                regex::Regex::new(std::str::from_utf8(&next_regex_string).unwrap()).unwrap();
        }

        self.current_regex.clone()
    }
    pub fn _print_bad_guesses(&self) {
        println!("{}", self.wrong_guesses)
    }
    pub fn already_lost(&self) -> bool {
        self.wrong_guesses >= self.guess_limit
    }
}
