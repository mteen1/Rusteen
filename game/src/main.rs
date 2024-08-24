use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("the secret number is {secret_num}");
    println!("Please input your guess.");
    let mut guess: String= String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("failed to read line");
    println!("You guessed: {guess}");
    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
