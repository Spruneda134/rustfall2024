fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
    let mut word = s1.to_string();
    word.push_str(s2);
    return word;
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}