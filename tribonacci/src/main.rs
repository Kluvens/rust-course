use std::env;
use std::num::ParseIntError;

struct TribonacciError(String);

fn main() {
    let args: Vec<String> = env::args().collect();
    let error_message = String::from("Please enter a valid size");

    let size = match args.get(1) {
        Some(s) => s.parse::<usize>(),
        None => Ok(10),
    };

    if let Err(e) = compute_tribonacci(size, error_message) {
        println!("Error: {}", e.0)
    }
}

/// Computes the tribonacci sequence of a given size
/// Prints the sequence, and its sum
fn compute_tribonacci(
    size: Result<usize, ParseIntError>,
    // The error message your function should return
    // inside the `TribonacciError` struct
    error_msg: String,
) -> Result<(), TribonacciError> {
    let size = match size {
        Ok(size_result) => size_result,
        Err(_) => return Err(TribonacciError(error_msg)),
    };

    let mut tri_vec:Vec<u128> = vec![1, 1, 1];
    for i in 2..size-1 {
        tri_vec.push(tri_vec[i] + tri_vec[i-1] + tri_vec[i-2]);
    }
    println!("Values: {:?}", tri_vec);
    println!();

    let sum: u128 = tri_vec.iter().sum();
    println!("Sum: {}", sum);

    Ok(())
}
