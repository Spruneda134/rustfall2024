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

    let guess = [4, 7, 55, 47];
    
    loop {
        println!("Guess the secret number: ");


        if check_guess(guess[guesses], secret) == 0 {
            println!("You did it in {} guesses!", guesses+1);
            break; 
        } else if check_guess(guess[guesses], secret) == 1 {
            println!("Too high!");
        } else {
            println!("Too low!");
        }
        guesses += 1;
    }
}
