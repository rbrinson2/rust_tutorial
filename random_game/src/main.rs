use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number");
    println!("{secret_number}");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
    
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number between 1 and 100.");
                continue;
            }
        };
    
    
    
        println!("You guessed: {guess}");
    
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }



}