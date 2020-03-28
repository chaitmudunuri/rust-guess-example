use rand::Rng;
use std::cmp::Ordering;
use std::io;

// define a struct to hold guessing state
#[derive(Debug)]
struct GuessingState {
    tries: u32,
    min: u32,
    max: u32,
}

fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100 to be used as secret.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Initialize the GuessingState
    let mut guessing_state = GuessingState {
        tries: 0,
        min: 0,
        max: 100,
    };

    loop {
        let guess = read_input();
        println!("You guessed: {}", guess);

        if check_guess(guess, secret_number, &mut guessing_state) {
            break;
        }
    }
}

// Read input from standard input
fn read_input() -> u32 {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    return guess;
}

// Check the guess against the secret
fn check_guess(guess: u32, secret: u32, gs: &mut GuessingState) -> bool {
    match guess.cmp(&secret) {
        Ordering::Less => {
            gs.tries += 1;
            gs.min = guess;
            println!("{:?} Too small!", gs);
            return false;
        }
        Ordering::Greater => {
            gs.tries += 1;
            gs.max = guess;
            println!("{:?} Too big!", gs);
            return false;
        }
        Ordering::Equal => {
            println!("Yay! {}. You got it in {} tries.", guess, gs.tries);
            return true;
        }
    }
}
