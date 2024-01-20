use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    prompt();

    let secret_number = gen_secret_number();

    let mut should_break = false;

    loop {

        let guess = read_guess();
       
        compare_guess_to_secret(&guess, &secret_number, &mut should_break);

        if should_break {
            break;
        }

    }
}

fn prompt() {
    println!("Guess the number!");
    println!("Please input your guess.");
}

fn gen_secret_number() -> i32 {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);
    return secret_number;
}

fn read_guess() -> i32 {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {

                if guess.trim() == "quit" {
                    println!("Quitting...");
                    std::process::exit(0);
                }

                println!("Please type a number!");
                read_guess()
            }
        };

    return guess;
}

fn compare_guess_to_secret(guess: &i32, secret_number: &i32, should_break: &mut bool) {
    let result_ordering = guess.cmp(&secret_number);

    match result_ordering {
        Ordering::Less => println!("Too low!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => {
            println!("You win!");
            *should_break = true;
        }
    }
}