#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use hangman_lib::*;

#[get("/<pattern>?<guesses>")]
fn index(pattern: String, guesses: Option<String>) -> String {
    let mut guesser = guessr::Guessr::default();
    guesser.filter_length(pattern.len());
    if let Some(guesses) = guesses {
        guesser.already_guessed(guesses.to_ascii_uppercase().as_bytes());
    }

    guesser.new_regex(&pattern);
    format!("{}", guesser.guess() as char)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
