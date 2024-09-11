fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        true
    }
    else{
        false
    }
}

fn main() {
    let nums =  [4, 47, 407, 1, 15, 3, 4, 5, 6, 1500];

    for num in nums.iter() {

        if num % 3 == 0 && num % 5 == 0 {
            println!("FizzBuzz!");
        }
        else if num % 3 == 0 {
            println!("Fizz!");
        }
        else if num % 5 == 0 {
            println!("Buzz!");
        }
        else {

            if is_even(*num){
                println!("{} is even!", num);
            }
            else{
                println!("{} is odd!", num);
            }

        }
    }
    let mut counter = 0;
    let mut total: i32 = 0;
    while counter < 10 {
        total += nums[counter];
        counter += 1;
    }
    println!("{} is the total!", total);

    let mut biggest = nums[0];
    counter = 1;
    loop {
        if nums[counter] > biggest {
            biggest = nums[counter];
        }
        counter += 1;

        if counter >= 10{
            break;
        }
    }
    println!("{} is the biggest!", biggest);


    
}
