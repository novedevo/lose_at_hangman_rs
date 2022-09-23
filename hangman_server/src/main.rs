use hangman_lib::*;
use rocket::{fs::NamedFile, get, launch, routes};
use std::path::Path;

static mut MAIN_GUESSER: guessr::Guessr = guessr::Guessr::const_default();

#[get("/api?<pattern>&<guesses>")]
fn api(pattern: String, guesses: Option<String>) -> String {
    let pattern = pattern.to_ascii_uppercase();
    let mut guesser = unsafe { MAIN_GUESSER.clone() };
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

#[get("/")]
async fn hangman() -> NamedFile {
    NamedFile::open(Path::new("hangman_server/hangman.html"))
        .await
        .unwrap()
}

#[launch]
fn rocket() -> _ {
    unsafe {
        MAIN_GUESSER = guessr::Guessr::new();
    }
    rocket::build().mount("/", routes![api, hangman])
}
