extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("\n---実行結果---");
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop{
        println!("Input your guess!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to read guess");

        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_)  => continue, 
        };

        println!("your guess is {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("too small!"),
            Ordering::Greater => println!("too great!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}