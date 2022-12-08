use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main(){
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("The generated number is {}", secret_number);
    loop {
        
    
    println!("Please input your number");
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Enter a string as input");
    println!("You guessed: {}", guess);
    let guess: u32= guess.trim().parse().expect("Please enter a string");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("The number u guessed is small"),
        Ordering::Greater => println!("The number  you guessed is bigger "),
        Ordering::Equal => {
            println!("You guessed the right number");
            break;
        }
    }
    }

}