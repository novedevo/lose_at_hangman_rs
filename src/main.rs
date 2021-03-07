mod guessr;
use std::collections::HashMap;

fn main() {
    test();
}

fn test() {
    
    let words: HashMap<String, f64> = HashMap::with_capacity(281700);
    
    let words = guessr::add_unordered(words, "data/words.txt").unwrap();
    let words = guessr::add_ordered(words, "data/English_Word_Prevalences.csv").unwrap();
    
    let words = guessr::filter_length(words, 5);
    
    let words = guessr::filter_regex(words, regex::Regex::new("....E").unwrap());
    
    println!("{:?}", words)
    
}
