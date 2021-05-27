pub mod executionr;
pub mod guessr;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let word = String::from("PINEAPPLE");

        let mut guesser = crate::guessr::Guessr::new();
        guesser.filter_length(word.len());
        let mut executioner = crate::executionr::Executionr::new(String::from(&word));
        while !guesser.already_won() && !executioner.already_lost() {
            let guess = guesser.guess();
            guesser.new_regex(executioner.execute(guess).as_str());
        }
        assert_eq!(word, guesser.final_answer());
    }

    // #[test]
    // fn serialize() {
    //     let guesser = crate::guessr::Guessr::_generate_new();
    //     guesser._serialize_hash_map();
    // }
}
