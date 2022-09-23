#[derive(Clone)]
pub struct Guessr {
    words: Vec<(String, f64)>,
    guesses: Vec<u8>,
    last_pattern: String,
}

impl Default for Guessr {
    fn default() -> Self {
        Self::new()
    }
}

impl Guessr {
    pub const fn const_default() -> Self {
        Self {
            words: vec![],
            guesses: vec![],
            last_pattern: String::new(),
        }
    }

    pub fn new() -> Self {
        Self {
            words: bincode::deserialize(include_bytes!("../../data/serialized_wordvec.bin"))
                .unwrap(),
            guesses: Vec::new(),
            last_pattern: String::new(),
        }
    }

    pub fn already_guessed(&mut self, guesses: &[u8]) {
        self.guesses.extend_from_slice(guesses);
    }

    pub fn guess(&mut self) -> u8 {
        let mut max = (b'#', 0.0);
        for (letter, frequency) in get_letter_frequencies(&self.words).iter().enumerate() {
            if max.1 < *frequency && !self.guesses.contains(&(letter as u8 + 65)) {
                max = (letter as u8 + 65, *frequency);
            }
        }
        self.guesses.push(max.0);
        max.0
    }

    pub fn new_regex(&mut self, pattern: &str) {
        if pattern != self.last_pattern {
            self.words = filter_regex(self.words.clone(), regex::Regex::new(&pattern).unwrap());
        }
    }

    pub fn already_won(&self) -> bool {
        self.words.len() == 1
    }

    pub fn gave_up(&self) -> bool {
        self.words.is_empty()
    }

    pub fn final_guess(&mut self) -> String {
        if self.words.is_empty() {
            String::from("I don't know your word. Are you sure you're using scrabble rules?")
        } else if self.words.len() == 1 {
            self.words[0].0.clone()
        } else {
            (self.guess() as char).to_string()
        }
    }

    pub fn final_answer(self) -> String {
        let mut max = 0.0;
        let mut retval = String::from("No idea :(");
        for key in self.words {
            if key.1 > max {
                retval = key.0;
                max = key.1;
            }
        }
        retval
    }

    pub fn get_remaining(self) -> Vec<(String, f64)> {
        self.words
    }

    pub fn filter_length(&mut self, length: usize) {
        self.words = self
            .words
            .clone()
            .into_iter()
            .filter(|(word, _)| word.len() == length)
            .collect();
    }
}

fn get_letter_frequencies(words: &[(String, f64)]) -> [f64; 26] {
    let mut frequencies = [0.0; 26];
    for (word, prevalence) in words {
        let charset: std::collections::BTreeSet<u8> = word.bytes().collect();
        for letter in charset {
            frequencies[letter as usize - 65] += *prevalence;
        }
    }
    frequencies
}

fn filter_regex(words: Vec<(String, f64)>, pattern: regex::Regex) -> Vec<(String, f64)> {
    words
        .into_iter()
        .filter(|(word, _)| pattern.is_match(word))
        .collect()
}
