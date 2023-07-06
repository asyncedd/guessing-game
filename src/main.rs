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

            println!("Let's play a game! I'm thinking of a number between {} and {}.", start, end);
            println!("You have 5 attempts to guess the number. Good luck!");

            let mut biggest_guess = end;
            let mut small_guess = start;

            while amount_of_guesses < 5 {
                amount_of_guesses += 1;
                let remaining_attempts = 5 - amount_of_guesses + 1;
                println!("\nAttempt {}/5 | Remaining attempts: {}", amount_of_guesses, remaining_attempts);
                let input = read_input("Enter your guess: ");

                match input.parse::<u32>() {
                    Ok(guess) => {
                        if guess == random_number {
                            println!("Congratulations! You guessed the correct number: {}", random_number);
                            return;
                        } else if guess < random_number {
                            if guess > small_guess {
                                small_guess = guess;
                            }
                            println!("Too low! The number is between {} and {}.", small_guess, biggest_guess);
                        } else {
                            if guess < biggest_guess {
                                biggest_guess = guess;
                            }
                            println!("Too high! The number is between {} and {}.", small_guess, biggest_guess);
                        }
                    }
                    Err(_) => {
                        println!("Invalid input. Please enter a valid number.");
                    }
                }
            }


            println!("\nGame over! You've reached the maximum number of guesses.");
            println!("The correct number was: {}", random_number);
        }
        _ => {
            println!("Invalid input. Please enter valid range values.");
            // Handle parsing errors here
        }
    }
}
