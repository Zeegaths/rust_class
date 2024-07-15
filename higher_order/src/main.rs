// Write a Rust program that creates a higher-order 
// function that takes a closure and a vector of numbers, 
// and applies the closure to each number in the vector,
//  returning a new vector of results.

fn main() {
    // Define a vector of numbers
    let numbers = vec![1, 2, 3, 4, 5];

    // Define a closure that doubles the input
    let double = |x: i32| x * 2;

    // Use the higher-order function to apply the closure to the vector
    let doubled_numbers = apply_to_each(numbers, double);

    // Print the resulting vector
    println!("{:?}", doubled_numbers);
}

// Define the higher-order function
fn apply_to_each<F>(numbers: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    // Apply the closure to each number in the vector and collect the results
    numbers.into_iter().map(f).collect()
}
