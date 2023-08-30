use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    const MAX_WINNING_TRIES: usize = 5;
    const SHOULD_SPOIL_SECRET: bool = true;

    let number = get_number(SHOULD_SPOIL_SECRET);

    let tries = keep_guessing(number);
    if tries == 0 {
        println!("You quit the game.");
    } else if tries <= MAX_WINNING_TRIES {
        println!("You win! It only took you {tries} tries to guess the correct number.");
    } else {
        println!("It took you {tries} tries to guess the correct number.");
    }
}

fn keep_guessing(expected: u32) -> usize {
    let mut tries: usize = 0;
    let mut low_guess: u32 = 1;
    let mut hi_guess: u32 = 100;

    loop {
        tries += 1;

        let guess = get_guess(low_guess, hi_guess);
        if guess == 0 {
            return guess as usize;
        }

        println!("You guessed: {guess}");
        if check_guess(guess, expected, &mut low_guess, &mut hi_guess) {
            break;
        }
    }

    tries
}

fn get_number(spoil_the_secret: bool) -> u32 {
    let number: u32 = rand::thread_rng().gen_range(1..=100);
    if spoil_the_secret {
        println!("Psst! The number is: {number}");
    }

    number
}

fn get_guess(low_guess: u32, hi_guess: u32) -> u32 {
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guess = guess.trim().to_lowercase().to_string();

        match guess.as_str() {
            "help" | "help?" | "hint" | "hint?" | "?" => show_help(low_guess, hi_guess),
            "quit" | "exit" | "stop" => break,
            _ => {
                if let Ok(num) = guess.parse() {
                    return num;
                }

                println!("Please enter a valid number!");
            }
        }
    }

    0
}

fn check_guess(guess: u32, expected: u32, low: &mut u32, hi: &mut u32) -> bool {
    match guess.cmp(&expected) {
        Ordering::Less => {
            println!("A cold wind blows over your left shoulder");
            *low = guess + 1;
            false
        }
        Ordering::Greater => {
            println!("A cold wind blows over your right shoulder");
            *hi = guess - 1;
            false
        }
        Ordering::Equal => true,
    }
}

fn show_help(low_guess: u32, hi_guess: u32) {
    let rand_guess = rand::thread_rng().gen_range(low_guess..hi_guess);
    let mid = (low_guess + hi_guess) / 2;

    println!("The number is between {low_guess} and {hi_guess}, e.g. {rand_guess} or {mid}");
}
