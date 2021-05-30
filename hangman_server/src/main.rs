#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use hangman_lib::*;
use rocket::response::NamedFile;
use std::path::Path;

#[get("/hangman/api?<pattern>&<guesses>")]
fn api(pattern: String, guesses: Option<String>) -> String {
    let pattern = pattern.to_ascii_uppercase();
    let mut guesser = guessr::Guessr::default();
    guesser.filter_length(pattern.len());
    let pattern = if let Some(guesses) = guesses {
        if !guesses.is_empty() {
            let guesses = guesses.to_ascii_uppercase();
            guesser.already_guessed(guesses.as_bytes());
            pattern.replace('.', &format!("[^{}]", guesses))
        } else {
            pattern
        }
    } else {
        pattern
    };

    guesser.new_regex(&format!("^{}$", pattern));
    guesser.final_guess()
}

#[get("/hangman")]
fn hangman() -> NamedFile {
    NamedFile::open(Path::new(
        "hangman_server/hangman.html",
    ))
    .unwrap()
}

fn main() {
    rocket::ignite().mount("/", routes![api, hangman]).launch();
}
