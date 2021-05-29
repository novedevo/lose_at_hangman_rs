#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use hangman_lib::*;

#[get("/hangman/api?<pattern>&<guesses>")]
fn api(pattern: String, guesses: Option<String>) -> String {
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

    guesser.new_regex(&format!("^{}$", pattern));
    guesser.final_guess()
}

#[get("/hangman")]
fn hangman() -> String {
    String::from("nothing here lol")
}

fn main() {
    rocket::ignite().mount("/", routes![api, hangman]).launch();
}
