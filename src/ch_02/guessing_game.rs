use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn play(prompt: &str) {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("Guess the number!");
    
    loop {
        let guess: u32 = input_u32(prompt);
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

pub fn input_u32(prompt: &str) -> u32 {
    loop {
        match cli_input(prompt).trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter an integer only");
                continue;
            }
        };
    }
}

pub fn cli_input(prompt: &str) -> String {
    let mut guess = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess
}
