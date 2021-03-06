extern crate csv;
use std::collections::HashMap;

pub struct Guessr {
    scrabble_dict_string: String,
    alphabet: String,
    pub words: HashMap<String, f64>,
}

impl Guessr {
    pub fn guessr_builder() -> Guessr {
        let mut retval = Guessr {
            scrabble_dict_string: std::fs::read_to_string("data/words.txt").expect("File not found"),
            alphabet: String::from("ESIARNOTLCDUPMGHBYFKVWZXQJ-"),
            words: Guessr::read_ordered_words(),
        };

        println!("hashmap created");
        retval.add_scrabble();

        retval
    }

    fn read_ordered_words() -> HashMap<String, f64> {
        let mut ret_map = HashMap::new();

        let mut rdr = csv::Reader::from_path("data/English_Word_Prevalences.csv").unwrap();
        type Record = (String, f64, u32, f64, f64);
        for result in rdr.deserialize() {
            let record: Record = match result {
                Ok(r) => r,
                _ => continue,
            };
            ret_map.insert(record.0.to_uppercase(), record.1);
        }

        ret_map
    }

    fn add_scrabble(&mut self) {
        for line in self.scrabble_dict_string.lines() {
            if !self.words.contains_key(line) {
                self.words.insert(String::from(line), 0.01);
            }
        }
    }
}
