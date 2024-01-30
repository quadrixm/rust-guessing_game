use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Enter 0 to exit the game");

    let secret_number: i8 = rand::thread_rng().gen_range(1..=100);
    let mut trial: i8 = 0;
    const MAX_TRAIL: i8 = 100;

    loop {
        
        println!("Please input your guess");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid number between 1 to 100 and 0 to exit");
                continue;
            },
        };

        if guess == 0 {
            println!("Exit successfully");
            break;
        }
    
        println!("You guessed : {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win in {trial} trails.");
                break;
            },
        }
        trial += 1;
        if trial >= MAX_TRAIL {
            println!("Max Attempt reached");
            break;
        }
    }

}
