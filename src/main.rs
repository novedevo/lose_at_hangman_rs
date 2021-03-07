mod guessr;

fn main() {
    test();
}

fn test() {
    
    let guesser = guessr::Guessr::guessr("data/English_Word_Prevalences.csv", "data/words.txt", ".....");
    
    println!("{:?}", guesser)
    
}
