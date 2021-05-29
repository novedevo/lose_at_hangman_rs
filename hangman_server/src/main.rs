#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use hangman_lib::*;

#[get("/<pattern>?<guesses>")]
fn index(pattern: String, guesses: Option<String>) -> String {
    let pattern = pattern.to_ascii_uppercase();
    let mut guesser = guessr::Guessr::default();
    guesser.filter_length(pattern.len());
    let pattern = if let Some(guesses) = guesses {
        let guesses = guesses.to_ascii_uppercase();
        guesser.already_guessed(guesses.as_bytes());
        pattern.replace('.', &format!("[^{}]", guesses))
    } else {
        pattern
    };

    // let stripped_pattern = pattern.to_ascii_uppercase().replace('.', "");

    // let pattern = if charset.is_empty() {
    //     pattern.to_string()
    // } else {
    //     pattern.replace('.', &format!("[^{}]", charset)).replace('-', r"\-")
    // };

    guesser.new_regex(&format!("^{}$", pattern));
    format!("{}", guesser.guess())
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
