extern crate rand;
use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
  println!("Guess the number!");
  let mut guesses = 0;
  let secret_number = rand::thread_rng().gen_range(1, 101);
  
  loop {
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
	Ok(n) => n,
	Err(_) => continue
    };

    guesses = guesses + 1;

    match guess.cmp(&secret_number) {
      Ordering::Less    => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal   => {
	println!("You win!, It took you {} tries", guesses);
	break;
      }
    }
  }
}
