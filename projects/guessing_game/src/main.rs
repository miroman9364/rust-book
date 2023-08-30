use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    const MAX_WINNING_TRIES: usize = 3;
    const SHOULD_SPOIL_SECRET: bool = true;

    let number = get_number(SHOULD_SPOIL_SECRET);

    let tries = keep_guessing(&number);
    if tries <= MAX_WINNING_TRIES {
        println!("You win! It only took you {tries} tries to guess the correct number.");
    } else {
        println!("It took you {tries} tries to guess the correct number.");
    }
}

fn keep_guessing(expected: &String) -> usize {
    let mut tries: usize = 0;

    loop {
        tries += 1;

        let guess = get_guess();

        println!("You guessed: {guess}");
        if check_guess(&guess, expected) {
            break;
        }
    }

    tries
}

fn get_number(spoil_the_secret: bool) -> String {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let number = format!("{secret_number}\n");
    if spoil_the_secret {
        print!("Psst! The number is: {number}");
    }

    number
}

fn get_guess() -> String {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess
}

fn check_guess(guess: &String, expected: &String) -> bool {
    if guess == expected {
        return true;
    }

    print!("Wrong! ");
    false
}
