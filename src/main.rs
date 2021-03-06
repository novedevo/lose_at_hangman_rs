mod guessr;

fn main() {
    test();
}

fn test() {
    let guesser = guessr::Guessr::guessr_builder();
    println!("{}", guesser.words.len());
}
