// imports Rng function from rand library
use rand::Rng;
// imports Ordering for size comparisons from the standard library
use std::cmp::Ordering;
// imports IO from standard for recieving and handling user input
use std::io;

fn main() {
    // prints a string to STDOUT
    println!("Guess the number!");

    // creates an immutable variable 'secret_number' and sets it to the output of
    // rand::thread_rng()
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("You have five tries to guess the number between 1 and 100.");

    for count in (1..=5).rev() {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // any function that can fail returns a 'Result' enum, which can be 'Ok(T)' with a value of
        // type T, or Err(E) with a value of E. T will be the type of the successful value, E will
        // be the type of error.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("You have {} tries left", count - 1);

        if count == 1 {
            println!("The number was {secret_number}. You lose!");
        }
    }
}
