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
    let num3 = nums[2];

    if num1 <= num2 && num1 <= num3 {
        println!("Min = {}", num1);
    } else if num2 <= num1 && num2 <= num3 {
        println!("Min = {}", num2);
    } else {
        println!("Min = {}", num3);
    }

    if num1 >= num2 && num1 >= num3 {
        println!("Max = {}", num1);
    } else if num2 >= num1 && num2 >= num3 {
        println!("Max = {}", num2);
    } else {
        println!("Max = {}", num3);
    }
}
