use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    const MAX_WINNING_TRIES: usize = 5;
    const SHOULD_SPOIL_SECRET: bool = true;

    let number = get_number(SHOULD_SPOIL_SECRET);

    let tries = keep_guessing(number);
    if tries <= MAX_WINNING_TRIES {
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

        let guess = get_guess();

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

fn get_guess() -> u32 {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess.trim().parse().expect("Please type a number!")
}

fn check_guess(guess: u32, expected: u32, low: &mut u32, hi: &mut u32) -> bool {
    let is_match = match guess.cmp(&expected) {
        Ordering::Less => {
            println!("A cold wind blows over your left shoulder");
            *low = guess;
            false
        }
        Ordering::Greater => {
            println!("A cold wind blows over your right shoulder");
            *hi = guess;
            false
        }
        Ordering::Equal => true,
    };

    if !is_match {
        let rand_guess = rand::thread_rng().gen_range((*low + 1)..(*hi - 1));
        let mid = (*low + *hi) / 2;
        println!("The secret number is between {low} and {hi}, e.g. {rand_guess} or {mid}");
    }

    is_match
}
