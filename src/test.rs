use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the guess game!!!");

    //Get the secret random number
    let secret: u32 = rand.thread_rng.gen_range(1..=100);

    loop {
        //Initialize the guess variable
        let mut guess = String::new();

        //Get the input from the user
        io.stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //Parse the user string input to u32
        let guess = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //Now make a comparison and print based on the ordering
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }

    }
}