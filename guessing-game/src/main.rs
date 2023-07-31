use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Enter Your Guess (between 1 - 100)");

    let random_number: u8 = rand::thread_rng().gen_range(1..=100);

    let mut tries_left: u8 = 7;

    loop {
        let mut users_guess: String = String::new();

        io::stdin()
            .read_line(&mut users_guess)
            .expect("Couldn't Read User's Input");

        let users_guess: u8 = match users_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter Your Guess (between 1 - 100)");
                continue;
            },
        };

        match users_guess.cmp(&random_number) {
            Ordering::Less => println!("Guess is small (Tries Left: {})", tries_left - 1),
            Ordering::Equal => println!("Horray!, You guessed it right"),
            Ordering::Greater => println!("Guess is big (Tries Left: {})", tries_left - 1),
        }

        if users_guess == random_number || tries_left <= 0 {
            if tries_left <= 0 {
                println!("Actual Number was {random_number}")
            }
            break;
        } else {
            tries_left -= 1;
        }
    }
}
