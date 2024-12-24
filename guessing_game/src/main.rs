use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

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

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("The number was {}, you win!", secret_number);
                break;
            }
        }
    }
}
// Notes & Comments
// Guess -- String is mutable so we can handle multiple guesses
// interesting, but obvoius, we denote function arguments as being mutable or not
// Guess -- we access the io library, call stdin function, 
// and the next lines are like members to handle reading stream (object?)
// Calling multiple methods with one statement and using whitespace for readability
