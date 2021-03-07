use std::collections::HashMap;
use std::error::Error;

const ALPHABET: &str = "ESIARNOTLCDUPMGHBYFKVWZXQJ-"; //sorted by how common they appear in the scrabble dictionary

#[derive(Debug)]
pub struct Guessr {
    words: HashMap<String, f64>,
    guesses: Vec<char>,
    last_regex: regex::Regex,
    last_guess: Option<char>,
}

impl Guessr {
    pub fn guessr(
        ordered_word_file: &str,
        unordered_word_file: &str,
        blank_slate: &str,
    ) -> Result<Guessr, Box<dyn Error>> {
        Ok(Guessr {
            words: filter_length(
                add_ordered(
                    add_unordered(HashMap::with_capacity(300_000), unordered_word_file)?,
                    ordered_word_file,
                )?,
                blank_slate.len(),
            ),
            guesses: Vec::new(),
            last_regex: regex::Regex::new(blank_slate).unwrap(),
            last_guess: None,
        })
    }

    pub fn new_regex(&mut self, pattern: regex::Regex) {
        if pattern.as_str() == self.last_regex.as_str() {
            //you really shouldn't have to convert them to strings
            self.words = filter_out_letter(self.words.clone(), self.last_guess.unwrap());
            //clone feels like an antipattern here
        } else {
            self.words = filter_regex(self.words.clone(), pattern);
        }
        if self.already_won() {}
    }
    pub fn already_won(&self) -> bool {
        self.words.len() == 1
    }
}

fn get_letter_prevalences(words: &HashMap<String, f64>) -> HashMap<char, u32> {
    let mut prevalences: HashMap<char, u32> = ALPHABET.chars().zip(std::iter::repeat(0)).collect();
    for word in words.keys() {
        let charset: std::collections::BTreeSet<char> = word.chars().collect();
        for letter in  charset{
            *prevalences.entry(letter).or_default() += 1;
        }
    }
    prevalences
}

fn add_ordered(mut words: HashMap<String, f64>, csv_path: &str) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    //honestly I have no idea what a dyn Error is

    let mut rdr = csv::Reader::from_path(csv_path)?; //passes errors to caller
    type Record = (String, f64, u32, f64, f64); //structure of the csv

    for result in rdr.deserialize() {
        let record: Record = result?;
        words.insert(record.0.to_uppercase(), record.1);
    }

    Ok(words)
}

fn add_unordered(mut words: HashMap<String, f64>, wordfile_path: &str) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    //honestly I have no idea what a dyn Error is

    for line in std::fs::read_to_string(wordfile_path)?.lines() {
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

fn filter_out_letter(words: HashMap<String, f64>, letter: char) -> HashMap<String, f64> {
    let mut filtered_words = HashMap::with_capacity(words.len() / 2);
    for (word, prevalence) in words {
        if !word.contains(letter) {
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
