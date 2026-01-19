use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let values: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Not a Number"))
        .collect();
    let x = values[0];
    let y = values[1];

    if x == 0 && y == 0 {
        println!("Origin");
    } else if x != 0 && y == 0 {
        println!("X axis");
    } else if y != 0 && x == 0 {
        println!("Y axis");
    } else if x > 0 && y > 0 {
        println!("1st Quadrant");
    } else if x < 0 && y > 0 {
        println!("2nd Quadrant");
    } else if x < 0 && y < 0 {
        println!("3rd Quadrant");
    } else if x > 0 && y < 0 {
        println!("4th Quadrant");
    }
}
