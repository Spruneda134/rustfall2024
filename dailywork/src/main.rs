fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn mult(x: i32, y: i32) -> i32 {
    let mut total: i32 = 0;
    for i in 0..y {
        total = add(total, x);
    }
    total
}

fn exp(z: i32) -> i32 {
    let mut total: i32 = 1;
    for i in 1..z+1 {
        total = mult(total, i);
    }
    total
}


fn main() {
    println!("2 + 2 = {}", add(2, 2));
    println!("3 * 12 = {}", mult(3, 12));
    println!("5! = {}", exp(5));
    println!("5 + 1/3 = {}", add(5, 1/3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
fn test_add_multiple() {
    let test_cases = vec![
        (1, 1, 2),
        (0, 0, 0),
        (-1, 1, 0),
        (100, -50, 50)
    ];
    
    for (a, b, expected) in test_cases {
        assert_eq!(add(a, b), expected, "Failed on input ({}, {})", a, b);
    }
}

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_mult() {
        assert_eq!(mult(2, 3), 6);
    }

    fn test_mult_neg() {
        assert_eq!(mult(2, -3), -6);
    }

    #[test]
    fn test_exp() {
        assert_eq!(exp(6), 720);
    }

    
}