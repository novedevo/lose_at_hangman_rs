extern crate csv;
use std::collections::HashMap;

// pub struct Guessr {
//     scrabble_dict_string: String,
//     alphabet: String,
//     pub words: HashMap<String, f64>,
// }

// impl Guessr {
//     pub fn guessr_builder() -> Guessr {
//         let mut retval = Guessr {
//             scrabble_dict_string: std::fs::read_to_string("data/words.txt").expect("File not found"),
//             alphabet: String::from("ESIARNOTLCDUPMGHBYFKVWZXQJ-"),
//             words: HashMap::new()
//         };

//         println!("hashmap created");
//         retval.add_scrabble();
//         println!("scrabblers added");
//         retval.add_ordered();
//         println!("orders added");

//         retval
//     }

//     fn add_ordered(&mut self) {
//         let mut rdr = csv::Reader::from_path("data/English_Word_Prevalences.csv").unwrap();
//         type Record = (String, f64, u32, f64, f64);
//         for result in rdr.deserialize() {
//             let record: Record = match result {
//                 Ok(r) => r,
//                 _ => continue,
//             };
//             self.words.insert(record.0.to_uppercase(), record.1);
//         }
//     }

//     fn add_scrabble(&mut self) {
//         for line in self.scrabble_dict_string.lines() {
//             self.words.insert(String::from(line), 0.01);
//         }
//     }
// }

pub fn add_ordered(
    mut words: HashMap<String, f64>,
    csv_path: &str,
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(csv_path).unwrap();

    type Record = (String, f64, u32, f64, f64);

    for result in rdr.deserialize() {
        let record: Record = result?;
        words.insert(record.0.to_uppercase(), record.1);
    }
    Ok(words)
}

pub fn add_unordered(
    mut words: HashMap<String, f64>,
    wordfile_path: &str,
) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    for line in std::fs::read_to_string(wordfile_path)?.lines() {
        words.insert(String::from(line), 0.01);
    }
    Ok(words)
}
