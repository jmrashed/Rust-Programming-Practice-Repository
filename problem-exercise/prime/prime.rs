use std::io;

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..=(n / 2) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let number: u32 = input.trim().parse().expect("Invalid input.");

    if is_prime(number) {
        println!("The number is prime.");
    } else {
        println!("The number is not prime.");
    }
    io::stdin().read_line(&mut input).expect("Failed to read input.");
}
