use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Not a Number"))
        .collect();

    let num1 = nums[0];
    let num2 = nums[1];
    let sum_last_digits = num1 % 10 + num2 % 10;

    println!("{}", sum_last_digits);
}
