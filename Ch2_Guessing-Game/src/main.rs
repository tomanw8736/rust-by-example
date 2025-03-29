// Standard crate
use std::io;
use std::cmp::Ordering;

// External crates
use rand::Rng;

fn read_input() -> String{
    println!("Guess the number!");
    println!("Please input your guess:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    return guess;
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let input: String = read_input();

        // Checking if the input isn't a valid number
        // and continuing and ignoring the error if it is
        let input: u32 = match input.trim().parse() {
            Ok(num) => num, // if `input.trim().parse()` runs successfully
            Err(_) => continue, // if it doesn't run successfully, it continues anyway
        };

        // Checking if the guess is too small, too big or if they win
        // `&secret_number` is used instead of `&mut secret_number`
        // because it's being used to compare instead of changing
        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // if the guess is too small
            Ordering::Greater => println!("Too big!"), // if the guess is too big
            Ordering::Equal => {  // if they won!
                println!("You win!");
                break;
            },
        }
    }
}