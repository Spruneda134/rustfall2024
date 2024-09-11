use std::io;  // Importing the necessary library for input/output

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret = 47; 
    let mut guesses = 0;
    
    loop {
        println!("Guess the secret number: ");

        let mut guess_str = String::new();
        
        io::stdin().read_line(&mut guess_str);

        let guess: i32 = guess_str.trim().parse().unwrap();

        guesses += 1;

        if check_guess(guess, secret) == 0 {
            println!("You did it in {} guesses!", guesses);
            break; 
        } else if check_guess(guess, secret) == 1 {
            println!("Too high!");
        } else {
            println!("Too low!");
        }
    }
}
