use std::fs::File;
use std::io::{self, Read, Write};

struct Car {
    make: String,
    model: String,
    year: u32,
}

fn reading_from_console() {
    let mut buffer = String::new();

    print!("What's your car make? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let make = buffer.trim().to_string();

    buffer.clear();

    print!("What's your car model? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();

    buffer.clear();

    print!("what year is it? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year = buffer.trim().parse().unwrap();

    let car = Car { make, model, year };

    let mut file = File::create("user_info.txt").unwrap();
    writeln!(file, "Your car is a {} {} {}!", car.year, car.make, car.model).unwrap();
    println!("Your car is a {} {} {}!", car.year, car.make, car.model);

    let mut file = File::open("user_info.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("File contents:\n{}", contents);
}

fn main(){
    reading_from_console();
}