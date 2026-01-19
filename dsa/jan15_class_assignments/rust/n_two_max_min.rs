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

    if num1 < num2 {
        println!("Min = {}", num1);
        println!("Max = {}", num2);
    } else {
        println!("Min = {}", num2);
        println!("Max = {}", num1);
    }
}
