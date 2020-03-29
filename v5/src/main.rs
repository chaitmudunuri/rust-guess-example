fn main() {
    println!("Guess the number!");
    let mut guessing_state = guess::GuessingState::new(1, 100);
    while !guessing_state.input().ok() {}
}
