use std::{
    fs::File,
    io::{BufWriter, Write},
};

use rustc_hash::FxHashMap;

const ALPHABET: &str = "ESIARNOTLCDUPMGHBYFKVWZXQJ-"; //sorted by how common they appear in the scrabble dictionary
                                                      // const initial_map: FxHashMap<String, f64> =

pub struct Guessr {
    words: FxHashMap<String, f64>,
    guesses: Vec<char>,
    last_pattern: String,
    pub last_guess: char,
}

impl Default for Guessr {
    fn default() -> Self {
        Self::new()
    }
}

impl Guessr {
    pub fn _generate_new() -> Self {
        let mut words = FxHashMap::default();
        _add_unordered(&mut words, include_str!("../data/words.txt"));
        _add_ordered(&mut words, include_str!("../data/ordered_words.csv"));
        Self {
            words,
            guesses: Vec::new(),
            last_pattern: String::new(),
            last_guess: '\0',
        }
    }

    pub fn new() -> Self {
        Self {
            words: bincode::deserialize(include_bytes!("../data/serialized_hashmap.bin")).unwrap(),
            guesses: Vec::new(),
            last_pattern: String::new(),
            last_guess: '\0',
        }
    }

    pub fn _serialize_hash_map(self) {
        let serialized = bincode::serialize(&self.words).unwrap();

        let file = File::create("data/serialized_hashmap.bin").expect("failed to create file");
        let mut buf = BufWriter::new(file);
        buf.write_all(&serialized).expect("failed to write to buffer");
        buf.flush().expect("failed to flush buffer")
    }

    pub fn guess(&mut self) -> char {
        let mut max: (char, u32) = ('#', 0);
        for (letter, prevalence) in get_letter_prevalences(&self.words) {
            if max.1 < prevalence && !self.guesses.contains(&letter) {
                max = (letter, prevalence);
            }
        }
        self.last_guess = max.0;
        self.guesses.push(max.0);
        max.0
    }
    pub fn new_regex(&mut self, pattern: &str) {
        //guards against additional characters of the previous guess in the wrong places
        self.words = filter_letter_count(
            self.words.clone(), //feels like an antipattern
            self.last_guess,
            pattern.matches(self.last_guess).count(),
        );

        if pattern == self.last_pattern {
            //you really shouldn't have to convert them to strings
        } else {
            self.words = filter_regex(self.words.clone(), regex::Regex::new(pattern).unwrap());
        }
    }

    pub fn already_won(&self) -> bool {
        self.words.len() == 1
    }

    pub fn gave_up(&self) -> bool {
        self.words.is_empty()
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

    pub fn get_remaining(self) -> FxHashMap<String, f64> {
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

fn get_letter_prevalences(words: &FxHashMap<String, f64>) -> FxHashMap<char, u32> {
    let mut prevalences: FxHashMap<char, u32> = ALPHABET.chars().zip(std::iter::repeat(0)).collect();
    for word in words.keys() {
        let charset: std::collections::BTreeSet<char> = word.chars().collect();
        for letter in charset {
            *prevalences.entry(letter).or_default() += 1;
        }
    }
    prevalences
}

fn _add_ordered(words: &mut FxHashMap<String, f64>, csv_string: &str) {
    let mut rdr = csv::Reader::from_reader(csv_string.as_bytes());
    type Record = (String, f64, u32, f64, f64); //structure of the csv

    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        words.insert(record.0.to_uppercase(), record.1);
    }
}

fn _add_unordered(words: &mut FxHashMap<String, f64>, words_string: &str) {
    for line in words_string.lines() {
        words.insert(String::from(line), 0.01);
    }
}

fn filter_letter_count(words: FxHashMap<String, f64>, letter: char, letter_count: usize) -> FxHashMap<String, f64> {
    words
        .into_iter()
        .filter(|(word, _)| word.matches(letter).count() == letter_count)
        .collect()
}

fn filter_regex(words: FxHashMap<String, f64>, pattern: regex::Regex) -> FxHashMap<String, f64> {
    words.into_iter().filter(|(word, _)| pattern.is_match(word)).collect()
}
