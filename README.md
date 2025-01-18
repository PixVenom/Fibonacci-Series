# Fibonacci Series in Rust

This program calculates and prints the Fibonacci series up to a user-specified number using Rust. It demonstrates the use of recursion to compute Fibonacci numbers and basic input/output operations in Rust.

## How It Works
- The program prompts the user to enter a number.
- It calculates Fibonacci numbers starting from `Fib(0)` to `Fib(n-1)` using a recursive function.
- The results are displayed in the terminal.

## Code Explanation

### Input Handling
The program reads input from the user using the `std::io` library and parses it into a `u32` type:
```rust
let mut input: String = String::new();
io::stdin()
    .read_line(&mut input)
    .expect("Failed to take input");
let input: u32 = input.trim().parse().expect("Enter your input");
```

### Fibonacci Function
The Fibonacci sequence is calculated using a recursive function:
```rust
fn fibonacci(n: u32) -> u32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}
```
- **Base Case**: Returns 0 for `Fib(0)` and 1 for `Fib(1)`.
- **Recursive Case**: Returns the sum of the two preceding Fibonacci numbers.

### Main Function
The main function iterates from 0 to `n-1` and prints each Fibonacci number:
```rust
for num in 0..input {
    println!("Fib({}) = {}", num, fibonacci(num));
}
```

## Usage
1. Compile the program using the Rust compiler:
   ```sh
   rustc filename.rs
   ```
2. Run the executable:
   ```sh
   ./filename
   ```
3. Enter the desired number when prompted.

### Example Output
```
Enter the number for which you the fibonacci series:
5
Fib(0) = 0
Fib(1) = 1
Fib(2) = 1
Fib(3) = 2
Fib(4) = 3
```

## Notes
- This implementation uses recursion, which may be inefficient for large numbers due to repeated calculations.
- For better performance, consider using memoization or an iterative approach.

## License
This code is open-source and available for use under the MIT License.
