import * as wasm from "lose_at_hangman";

const guesser = wasm.Guessr.new(".........");

console.log(guesser.guess());