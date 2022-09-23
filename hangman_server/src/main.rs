use hangman_lib::guessr::Guessr;
use lazy_static::lazy_static;
use rocket::{get, launch, response::content::RawHtml, routes};

lazy_static! {
    static ref MAIN_GUESSER: Guessr = Guessr::new();
}

#[get("/api?<pattern>&<guesses>")]
fn api(pattern: String, guesses: Option<String>) -> String {
    let pattern = pattern.to_ascii_uppercase();
    let mut guesser = MAIN_GUESSER.clone();
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
async fn hangman() -> RawHtml<&'static str> {
    RawHtml(include_str!("../hangman.html"))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![api, hangman])
}
