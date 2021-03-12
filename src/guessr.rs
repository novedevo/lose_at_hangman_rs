use std::collections::HashMap;
use std::error::Error;
use wasm_bindgen::prelude::*;

use regex::Regex;

const ALPHABET: &str = "ESIARNOTLCDUPMGHBYFKVWZXQJ-"; //sorted by how common they appear in the scrabble dictionary

#[wasm_bindgen]
pub struct Guessr {
    words: HashMap<String, f64>,
    guesses: Vec<char>,
    last_regex: regex::Regex,
    last_guess: Option<char>,
}

#[wasm_bindgen]
impl Guessr {
    pub fn new(ordered_words_csv: &str, unordered_words: &str, blank_slate: &str) -> Guessr {
        Guessr {
            words: filter_length(
                add_ordered(
                    add_unordered(HashMap::with_capacity(300_000), unordered_words).unwrap(),
                    ordered_words_csv,
                )
                .unwrap(),
                blank_slate.len(),
            ),
            guesses: Vec::new(),
            last_regex: regex::Regex::new(blank_slate).unwrap(),
            last_guess: None,
        }
    }

    pub fn guess(&mut self) -> char {
        let mut max: (char, u32) = ('#', 0);
        for (letter, prevalence) in get_letter_prevalences(&self.words) {
            if max.1 < prevalence && !self.guesses.contains(&letter) {
                max = (letter, prevalence);
            }
        }
        self.last_guess = Some(max.0);
        self.guesses.push(max.0);
        max.0
    }
    pub fn new_regex(&mut self, pattern: &str) {
        //guards against additional characters of the previous guess in the wrong places
        let pattern = Regex::new(pattern).unwrap();
        self.words = filter_letter_count(
            self.words.clone(), //feels like an antipattern
            self.last_guess.unwrap(),
            pattern.as_str().matches(self.last_guess.unwrap()).count(),
        );

        if pattern.as_str() == self.last_regex.as_str() {
            //you really shouldn't have to convert them to strings
        } else {
            self.words = filter_regex(self.words.clone(), pattern);
        }

        if self.already_won() {}
        self.words.shrink_to_fit(); //conserve memory
    }

    pub fn already_won(&self) -> bool {
        self.words.len() == 1
    }

    fn _print_letter_frequencies(&self) {
        println!("{:?}", get_letter_prevalences(&self.words))
    }

    fn _print_wordlist(&self) {
        println!("{:?}", &self.words);
    }

    fn _print_last_guess(&self) {
        match self.last_guess {
            None => println!("No guesses have yet been made."),
            Some(l) => println!("{}", l),
        }
    }

    //consumes the guesser!
    pub fn final_answer(self) -> String {
        let max = 0.0;
        let mut retval = String::from("No idea :(");
        for key in self.words {
            if key.1 > max {
                retval = key.0;
            }
        }
        retval
    }
}

fn get_letter_prevalences(words: &HashMap<String, f64>) -> HashMap<char, u32> {
    let mut prevalences: HashMap<char, u32> = ALPHABET.chars().zip(std::iter::repeat(0)).collect();
    for word in words.keys() {
        let charset: std::collections::BTreeSet<char> = word.chars().collect();
        for letter in charset {
            *prevalences.entry(letter).or_default() += 1;
        }
    }
    prevalences
}

fn add_ordered(mut words: HashMap<String, f64>, csv_string: &str) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    //honestly I have no idea what a dyn Error is

    let mut rdr = csv::Reader::from_reader(csv_string.as_bytes()); //passes errors to caller
    type Record = (String, f64, u32, f64, f64); //structure of the csv

    for result in rdr.deserialize() {
        let record: Record = result?;
        words.insert(record.0.to_uppercase(), record.1);
    }

    Ok(words)
}

fn add_unordered(mut words: HashMap<String, f64>, words_string: &str) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    //honestly I have no idea what a dyn Error is

    for line in words_string.lines() {
        //passes IO errors back to caller
        words.insert(String::from(line), 0.01);
    }

    Ok(words)
}

//TODO: refactor these filters once I learn how to use closures

fn filter_length(words: HashMap<String, f64>, length: usize) -> HashMap<String, f64> {
    let mut filtered_words = HashMap::with_capacity(words.len() / 10);
    for (word, prevalence) in words {
        if word.len() == length {
            filtered_words.insert(word, prevalence);
        }
    }
    filtered_words
}

fn filter_letter_count(words: HashMap<String, f64>, letter: char, letter_count: usize) -> HashMap<String, f64> {
    let mut filtered_words = HashMap::with_capacity(words.len());
    for (word, prevalence) in words {
        if word.matches(letter).count() == letter_count {
            filtered_words.insert(word, prevalence);
        }
    }
    filtered_words
}

fn filter_regex(words: HashMap<String, f64>, pattern: regex::Regex) -> HashMap<String, f64> {
    let mut filtered_words = HashMap::with_capacity(words.len() / 10);
    for (word, prevalence) in words {
        if pattern.is_match(&word) {
            filtered_words.insert(word, prevalence);
        }
    }
    filtered_words
}
