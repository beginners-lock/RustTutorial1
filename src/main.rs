use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    fibonacci_generator();
}

fn fibonacci_generator() {
    println!("Welcome to the Fibonacci Generator");

    loop {
        println!("What fibonacci position would you like stop at or type 'exit' to stop process?");

        let mut position = String::new();

        io::stdin()
            .read_line(&mut position)
            .expect("Failed to read line");

        if position.trim()=="exit" {
            break;
        }

        let position: u32 = match position.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Position should be a number!!!");
                continue;
            }
        };

        if position == 0 {
            println!("Positioning is from 1 and above");
            continue;
        }

        println!("=> 0");
        if position==1 {
            println!("At position {position} we have 0");
            continue;
        }

        println!("=> 1");
        if position==2 {
            println!("At position {position} we have 1");
            continue;
        }

        println!("=> 1");
        if position==3 {
            println!("At position {position} we have 1");
            continue;
        }

        let mut prev_num = 1;
        let mut cur_num = 1;
        let mut cur_pos = 3;

        while cur_pos<=position {
            let result = cur_num + prev_num;
            prev_num = cur_num;
            cur_num = result;
            println!("=> {result}");
            cur_pos += 1;
        }

        println!("At position {position} we have {cur_num}");
    }
}

//Temperature converter
fn temp_converter() {
    const MAX_USER_PROMPT_NUM: u32 = 2;
    println!("Welcome to Temperature Converter!!!");
    temp_fn_instructions();

    loop {
        println!("What conversion would you like to do?");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                temp_fn_instructions();
                continue;
            },
        };

        if choice == 0 { //User wants to end the program
            break;
        }
        else if choice > MAX_USER_PROMPT_NUM{ //User inputted a number outside the 0-2 range
            println!("Invalid input!!!");
            temp_fn_instructions();
            continue;
        }
        else {
            println!("Input the temperature: ");
            let mut temperature_input = String::new();

            //'Take in the temperature as a user input (string)
            io::stdin()
                .read_line(&mut temperature_input)
                .expect("Failed to read line");

            //Convert this string to float cause there's a chance that the result is a decimal
            let temperature_input: f32 = match temperature_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid temprature entry (entry must be a number)!!!");
                    continue;
                }
            };

            //Depending on the choice C to F or F to C do the conversion
            if choice == 1 {
                let temperature_result = (temperature_input * 1.8) + 32.0;
                println!("{temperature_input} Celsius is equivalent to {temperature_result} Fahrenheit");
            }

            if choice == 2 {
                let temperature_result = (temperature_input - 32.0) / 1.8;
                println!("{temperature_input} Fahrenheit is equivalent to {temperature_result} Celsius");
            }
        }
    }
}

fn temp_fn_instructions(){
    println!("0 - End program");
    println!("1 - Celsius to Fahrenheit");
    println!("2 - Fahrenheit to Celsius");
}

//Guessing Game
fn guess_game() {
    println!("Welcome to Guess!!!");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        }
    }
}
