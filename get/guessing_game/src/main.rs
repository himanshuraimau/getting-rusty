use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(err) => {
                println!("Input Valid Number");
                continue;
            }
        }; //Shadowing
        println!("your guess is {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You WoN!");
                break;
            }
            Ordering::Greater => println!("It is greater"),
            Ordering::Less => println!("Too Small"),
        }
    }
    println!("Game Over!")
}
