const FPOINT: f64 = 32.0;

//Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    return (f- FPOINT) * 5.0/9.0;
}

// celsius_to_fahrenheit(c: f64) -> f64

fn main() {
    let mut temp: f64 = 99.0;
    let mut x = fahrenheit_to_celsius(temp);
    println!("Temperature in Celsius: {:.2}", x);

    loop {
        temp += 1.0;
        x = fahrenheit_to_celsius(temp);
        println!("Temperature in Celsius: {:.2}", x);

        if temp >= 104.0 {
            break;
        }


}
}