use std::collections::HashMap;
use std::error::Error;

const ALPHABET: &str = "ESIARNOTLCDUPMGHBYFKVWZXQJ-"; //sorted by how common they appear in the scrabble dictionary
                                                      // const initial_map: HashMap<String, f64> =

pub struct Guessr {
    words: HashMap<String, f64>,
    guesses: Vec<char>,
    last_pattern: String,
    pub last_guess: char,
}

impl Guessr {
    pub fn new(blank_slate: &str) -> Guessr {
        Guessr {
            words: filter_length(
                add_ordered(
                    add_unordered(HashMap::with_capacity(300_000), include_str!("../data/words.txt")).unwrap(),
                    include_str!("../data/ordered_words.csv"),
                )
                .unwrap(),
                blank_slate.len(),
            ),
            guesses: Vec::new(),
            last_pattern: String::new(),
            last_guess: '\0',
        }
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

    pub fn get_remaining(self) -> HashMap<String, f64> {
        self.words
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
    for line in words_string.lines() {
        //passes IO errors back to caller
        words.insert(String::from(line), 0.01);
    }

    Ok(words)
}

fn filter_length(words: HashMap<String, f64>, length: usize) -> HashMap<String, f64> {
    words.into_iter().filter(|(word, _)| word.len() == length).collect()
}

fn filter_letter_count(words: HashMap<String, f64>, letter: char, letter_count: usize) -> HashMap<String, f64> {
    words
        .into_iter()
        .filter(|(word, _)| word.matches(letter).count() == letter_count)
        .collect()
}

fn filter_regex(words: HashMap<String, f64>, pattern: regex::Regex) -> HashMap<String, f64> {
    words.into_iter().filter(|(word, _)| pattern.is_match(word)).collect()
}
