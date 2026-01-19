use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let marks: i32 = input.trim().parse().expect("Please type a number!");

    if marks > 90 {
        println!("Excellent");
    } else if marks > 80 && marks <= 90 {
        println!("Good");
    } else if marks > 70 && marks <= 80 {
        println!("Fair");
    } else if marks > 60 && marks <= 70 {
        println!("Meets Expectations");
    } else {
        println!("Below Par");
    }
}
