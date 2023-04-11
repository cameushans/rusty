extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let random_number:u32 = rand::thread_rng().gen_range(1, 100);
    let mut inp = String::new();
 
    io::stdin().read_line(&mut inp).expect("this is an error");
    let inp:u32 = inp.trim().parse().expect("please type a number");
    print!("Your Guess: {}", inp);
    println!("the secret number is: {}", random_number);
    match inp.cmp(&random_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}


