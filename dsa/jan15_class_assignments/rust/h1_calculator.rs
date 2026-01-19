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

    let n = nums[0];
    let m = nums[1];

    println!("{} + {} = {}", n, m, n + m);
    println!("{} - {} = {}", n, m, n - m);
    println!("{} * {} = {}", n * m);
    println!("{} / {} = {}", n / m);
    println!("{} % {} = {}", n % m);
}
