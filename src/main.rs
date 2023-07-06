mod input;

use rand::Rng;
use input::*;

fn main() {
    let input_one = read_input("Enter the first range (X-y): ");
    let input_two = read_input("Enter the second range (x-Y): ");

    let range_one: Result<u32, _> = input_one.parse();
    let range_two: Result<u32, _> = input_two.parse();

    match (range_one, range_two) {
        (Ok(start), Ok(end)) => {
            if start > end {
                println!("Invalid input. The second range must be smaller than the first.");
                return;
            }

            let mut rng = rand::thread_rng();
            let random_number: u32 = rng.gen_range(start..=end);
            let mut amount_of_guesses = 0;

            while amount_of_guesses < 5 {
                amount_of_guesses += 1;
                println!("Guess the number! ({}-{})", start, end);
                let input = read_input("Enter your guess: ");

                match input.parse::<u32>() {
                    Ok(guess) => {
                        if guess == random_number {
                            println!("Congratulations! You guessed the correct number: {}", random_number);
                            return;
                        } else if guess < random_number {
                            println!("Too low!");
                        } else {
                            println!("Too high!");
                        }
                    }
                    Err(_) => {
                        println!("Invalid input. Please enter a valid number.");
                    }
                }
            }

            println!("You've reached the maximum number of guesses. The correct number was: {}", random_number);
        }
        _ => {
            println!("Invalid input. Please enter valid range values.");
            // Handle parsing errors here
        }
    }
}
