use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
    println!("Guess a number");
    println!("Please input your number:");

    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)
        .expect("failed to read the line");
    let guess: u32 = guess.trim().parse().expect("failed to parse");
    println!("you guessed {}", guess);
    match guess.cmp(&secret_number) {
        Ordering::Less =>println!("too low"),
        Ordering::Greater =>println!("too high"),
        Ordering::Equal =>println!("Good job"),
    }
}