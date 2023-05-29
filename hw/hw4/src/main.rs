use std::ops::RangeInclusive;

fn main() {
    println!("Hello, fizzbuzz world!");
    let range: RangeInclusive<i32> = 1..=301;
    //let numbers: Vec<RangeInclusive<i32>> = vec![1..=301];
    let numbers: Vec<i32> = range.collect();
    
    println!("The total fizzbuzz count is: {}", fizzbuzz(numbers));
}

// Returns fizzbuzz count of supplied vector
fn fizzbuzz(numbers: Vec<i32>) -> i32 {
    let mut count: i32 = 0;
    for number in numbers.iter() {
        if number % 3 == 0 && number % 5 == 0 {
            count += 1;
            println!("fizzbuzz");
        } else if number % 3 == 0 && number % 5 > 0 {
            println!("fizz");
        } else if number % 3 > 0 && number % 5 == 0 {
            println!("buzz");
        }
    } 
    count
}
