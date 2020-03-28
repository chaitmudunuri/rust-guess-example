use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generate a random number between 1 and 100 to be used as secret.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Track number of tries. Declare mutable as we change value.
    let mut num_tries = 0;
    // Guessing range will be tracked using a tuple. Declare mutable as we change value.
    // min and max values
    let mut guessing_range = (1, 100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        num_tries = num_tries + 1; // not using += 1 for better readability

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                guessing_range.0 = guess;
                println!(
                    "Tries: {}. Range {:?}. Too small!",
                    num_tries, guessing_range
                );
            }
            Ordering::Greater => {
                guessing_range.1 = guess;
                println!("Tries: {}. Range {:?}. Too big!", num_tries, guessing_range);
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
