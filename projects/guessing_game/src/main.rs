use std::io;

fn main() {
    println!("Guess the number!");

    const MAX_WINNING_TRIES: usize = 3;
    let number: String = String::from("5\n");
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
        return true
    }

    print!("Wrong! ");
    false
}
