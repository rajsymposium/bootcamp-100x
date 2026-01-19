use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let marks: i32 = input.trim().parse().expect("Please type a number!");

    if marks < 35 {
        println!("Fail");
    } else {
        println!("Pass");
    }
}
