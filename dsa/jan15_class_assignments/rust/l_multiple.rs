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

    let number = nums[0];
    let multiple = nums[1];

    if multiple % number == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
