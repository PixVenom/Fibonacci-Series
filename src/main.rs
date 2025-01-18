use std::io;

fn main() {
    println!("Enter the number for which you the fibonacci series: ");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to take input");
    let input: u32 = input.trim().parse().expect("Enter your input");
    for num in 0..input {
        println!("Fib({}) = {}", num, fibonacci(num));
    }
}

fn fibonacci(n: u32) -> u32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}
