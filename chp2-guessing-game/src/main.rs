use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut number_of_guesses : u32 = 0;
    let mut number_of_bad_inputs : u32 = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        let result_of_input = io::stdin()
                                .read_line(&mut guess);

        number_of_guesses += 1;

        // Print out the contents of the Result<> enum
        // that is returned from std::io::stdin().read_line()
        // println!("Results of input:\n{:?}\n\n", result_of_input);
        
        // Throws exception if read_line contains an "Err"
        // This means the input could not be read.
        result_of_input.expect("Invalid input detected. What did you do, sneeze into your USB port?");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_error) => {
                print_insult(&number_of_bad_inputs);

                number_of_bad_inputs += 1;

                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("\nYou lucky bastard. You've done it!");
                println!("It took you {} {}.",
                    number_of_guesses,
                    if number_of_guesses == 1 { "attempt" } else { "attempts" }
                );

                break;
            },
        }
    }
}

fn print_insult(number_of_failures : &u32) {
    match number_of_failures {
        0 => println!("You need to enter a natural number from 1-100. I certainly hope you are capable of doing that..."),
        1 => println!("I spoke too soon. If ignorance is trulbliss, then you must be quite content."),
        2 => println!("There are more brains in a head of cabbage than your own."),
        3 => println!("You make me sick. ENTER A NATURAL NUMBER ALREADY!!!"),
        4 => {
            panic!("ERROR: User intelligence not found. Patience overflow detected. Aborting!");
        }
        _ => println!("You must be a QA engineer. Please accept my apologies.")
    }
}

