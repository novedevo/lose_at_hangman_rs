mod guessr;

use std::collections::HashMap;

fn main() {
    test();
}

fn test() {
    
    let words: HashMap<String, f64> = HashMap::new();
    
    let words = guessr::add_ordered(words, "data/English_Word_Prevalences.csv").unwrap();
    let words = guessr::add_unordered(words, "data/words.txt").unwrap();
    
}
