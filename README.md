# Rust Guessing Game 
## Learn Rust Basics by improving the game

Goal: Showcase various features of Rust by building and improvising the Guessing Game from Rust.
We start with the basic code from the **Rust Book** and add new data structures and functions to make the code modular and reusable.

Using `cargo search` to find the latest versions of the crates.
Crates are similar to packages in Java or modules in Python/Go.

* Use `cargo build` to build the code
* Use `cargo run` to run the code

**Version 1 - v1**  
This is straight copy from the Rust book.  
User is asked to guess a number between 1 and 100.
If the guess is bigger than secret, it says _Too big!_ and it small says _Too small!_

**Source**: [Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

**Version 2 - v2**
Version 1 just shows guidelines aorund guess (Too big or small).  
Version 2 will improvise by showing number of tries and guessing range to make it easy for user.

Added variables `num_tries` and `guessing_range`

`v2/src/main.rs`  
    let mut num_tries = 0;
    ...
    let mut guessing_range = (1, 100);

**Sample Output**
```
Please input your guess.
25
You guessed: 25
Tries: 2. Range (1, 25). Too big!
Please input your guess.
12
You guessed: 12
Tries: 3. Range (1, 12). Too big!
Please input your guess.
```