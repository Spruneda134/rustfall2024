fn main() {
    let x = 5;
    let y = x; // This creates a copy for primitive types
    println!("x is: {}, y is: {}", x, y);

    let s1 = String::from("Hello");
    let s2 = s1; // This moves ownership, s1 is no longer valid
    
    // println!("s1 is: {}, s2 is: {}", s1, s2); // This would cause a compile error
}