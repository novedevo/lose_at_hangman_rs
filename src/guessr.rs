use std::collections::HashMap;
use std::error::Error;

pub fn add_ordered(mut words: HashMap<String, f64>, csv_path: &str) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    //honestly I have no idea what a dyn Error is

    let mut rdr = csv::Reader::from_path(csv_path).unwrap();
    type Record = (String, f64, u32, f64, f64); //structure of the

    for result in rdr.deserialize() {
        let record: Record = result?;
        words.insert(record.0.to_uppercase(), record.1);
    }

    Ok(words)
}

pub fn add_unordered(
    mut words: HashMap<String, f64>,
    wordfile_path: &str,
) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    //honestly I have no idea what a dyn Error is

    for line in std::fs::read_to_string(wordfile_path)?.lines() {
        words.insert(String::from(line), 0.01);
    }

    Ok(words)
}
