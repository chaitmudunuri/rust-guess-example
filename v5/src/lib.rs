use rand::Rng;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Debug;
use std::io;

// define a struct to hold guessing state
#[derive(Debug)]
pub struct GuessingState {
    guess: u32,  // store the current guess
    tries: u32,  // number of tries
    min: u32,    // min value of range
    max: u32,    // max value of range
    secret: u32, // secret value
    left: u32,   // left indicator of range
    right: u32,  // right indicator of range
}

impl GuessingState {
    pub fn new(x: u32, y: u32) -> GuessingState {
        GuessingState {
            guess: 0,
            tries: 0,
            min: x,
            max: y,
            secret: rand::thread_rng().gen_range(x, y),
            left: x,
            right: y,
        }
    }

    // Check the guess against the secret
    pub fn ok(&mut self) -> bool {
        match self.guess.cmp(&self.secret) {
            Ordering::Less => {
                self.tries += 1;
                self.left = self.guess;
                return false;
            }
            Ordering::Greater => {
                self.tries += 1;
                self.right = self.guess;
                return false;
            }
            Ordering::Equal => {
                println!("Yay! You got it in {} tries.", self.tries);
                return true;
            }
        }
    }

    /// Read the guess from standard input.
    pub fn input(&mut self) -> &mut GuessingState {
        println!("Guess: {}", self);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Expecting a number.");

        let input = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        // Check and be in the guessing range
        if input > self.min && input < self.max {
            self.guess = input;
        }
        return self;
    }
}

// Implement Display trait for print
impl fmt::Display for GuessingState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for x in self.min..self.max + 1 {
            if x == self.left {
                buffer.push('[');
            } else if x == self.right {
                buffer.push(']');
            } else {
                buffer.push('=');
            }
        }
        write!(
            f,
            "[{} - {}] ({})\n{}",
            self.left, self.right, self.tries, buffer
        )
    }
}
