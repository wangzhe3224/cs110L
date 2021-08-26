// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::collections::HashSet;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("Random word: {}", secret_word);

    // Your code here! :)
    println!("Welcome to CS110L Hangman!");
    let mut guessed_info: Vec<LetterInfo> = secret_word
        .chars()
        .map(|c| LetterInfo {
            letter: c,
            guessed: false,
        })
        .collect();
    let mut guesses_left = 5;
    let guessed_letters: HashSet<char> = HashSet::new();

    while guesses_left > 0 {
        print_guessed_letters(&guessed_info);
        show_left(guesses_left);
        let guess =show_prompt();
        let won = show_hits(&guess, &mut guessed_info, &mut guesses_left);
        // debug
        // for i in guessed_info.iter_mut() {
        //     println!("gussed info {} {}", i.letter, i.guessed);
        // }
        if won {
            println!("Congratulations you guessed the secret word: {}!", secret_word);
            break;
        }
    }

    if guesses_left == 0 {
        println!("Sorry, you ran out of guesses!");
    }
}

fn print_guessed_letters(letters: &Vec<LetterInfo>) {
    print!("You have guessed the following letters: ");
    for l in letters {
        if l.guessed {
            print!("{}", l.letter)
        } else {
            print!("-")
        }
    }
    println!();
}

fn show_left(guesses: u8) {
    println!("You have {} guesses left.", guesses)
}

fn show_prompt() -> String {
    print!("Please guess a char: ");
    // Make sure the prompt from the previous line gets displayed:
    io::stdout().flush().expect("Error flushing stdout.");   
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading line.");

    guess
}

struct LetterInfo {
    letter: char,
    guessed: bool,
}

fn show_hits(guess: &str, letter_info: &mut Vec<LetterInfo>, guess_left: &mut u8) -> bool {
    let mut hit = false;
    let mut letters_remain = false;

    for letter in letter_info {
        if guess.chars().collect::<Vec<char>>()[0] == letter.letter {
            letter.guessed = true;
            hit = true;
        } else {
            if !letter.guessed {
                letters_remain = true;
            }
        }
    }

    if !hit {
        println!("Sorry, that letter is not in the word");
        *guess_left -= 1;
    }

    if !letters_remain {
        true
    } else {
        false
    }
}
