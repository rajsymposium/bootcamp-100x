use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num: i32 = input.trim().parse().expect("Please type a number!");

    println!("{} * 1 = {}", num, num * 1);
    println!("{} * 2 = {}", num, num * 2);
    println!("{} * 3 = {}", num, num * 3);
    println!("{} * 4 = {}", num, num * 4);
    println!("{} * 5 = {}", num, num * 5);
    println!("{} * 6 = {}", num, num * 6);
    println!("{} * 7 = {}", num, num * 7);
    println!("{} * 8 = {}", num, num * 8);
    println!("{} * 9 = {}", num, num * 9);
    println!("{} * 10 = {}", num, num * 10);
}
