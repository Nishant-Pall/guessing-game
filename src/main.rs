use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");
    println!("Input your guess");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please enter a valid number");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("To small"),
        Ordering::Greater => println!("To Big"),
        Ordering::Equal => println!("Yo win"),
    }
}
