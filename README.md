# lose_at_hangman_rs
Doing its best to beat you at hangman.

This project is hosted on Oracle Cloud at [hangman.nove.dev](https://hangman.nove.dev), cached and proxied by Cloudflare.

The next version of [Rocket](https://rocket.rs) will be compiled with stable Rust, but until then, this project needs to be compiled with a nightly toolchain.
This can be accomplished by running, e.g., `rustup default nightly`

Running `cargo run --bin hangman_server` will run the web server, which is the most ergonomic way to play hangman. 
Find it at localhost:8000/hangman in your browser of choice, or run it on a server, exposed to the world. 
It's up to you.
