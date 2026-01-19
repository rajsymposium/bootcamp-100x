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

    let length = nums[0];
    let width = nums[1];

    println!("Area = {}", length * width);
    println!("Perimeter = {}", 2 * (length + width));
}
