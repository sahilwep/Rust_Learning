use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Welcome to Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {}", secret_number);    // uncomment this to know guessed number.

    let mut cnt = 0;

    // loop to iterate until user wins the game
    loop {
        println!("Enter your guessed number: ");    
        let mut guess = String::new();  // Create a string to take input.
        
        // Taking input
        io::stdin()
        .read_line(&mut guess)  // reading line
        .expect("failed to read line"); // handling failure 
        
        cnt = cnt + 1;  // counter

        // parsing string to integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // handling error
            Err(_) => continue, // if error skip the guessing iteration.
        };
        
        println!("You guessed: {}", guess); // printing guessed number
        
        // match guess number with random generated number.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>  {
                println!("You win!");   // when user wins the loop will exit.
                break;
            }
        }
    }   
    println!("You have taken {} times to solve", cnt);
}