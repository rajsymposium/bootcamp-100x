use std::io;

fn main() {
    let mut first_input = String::new();
    io::stdin()
        .read_line(&mut first_input)
        .expect("Failed to read line");
    let n: i64 = first_input.trim().parse().expect("Please type a number!");

    let mut second_input = String::new();
    io::stdin()
        .read_line(&mut second_input)
        .expect("Failed to read line");
    let m: i64 = second_input.trim().parse().expect("Please type a number!");

    println!("{} + {} = {}\n", n, m, n + m);
    println!("{} - {} = {}\n", n, m, n - m);
    println!("{} * {} = {}\n", n, m, n * m);
    println!("{} / {} = {}\n", n, m, n / m);
    println!("{} % {} = {}", n, m, n % m);
}
