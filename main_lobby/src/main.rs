use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess how many players will be on this lobby");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("Failed to read the line");
    
        println!("You guessed: {}", guess);
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small, wont be fun"),
            Ordering::Greater => println!("Too big, skill issued guys will have a terrible day"),
            Ordering::Equal => {
                println!("Not that's a perfect lobby");
                break;
            }
        }

    }
}
