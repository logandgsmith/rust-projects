// Written by Logan D.G. Smith
// Solutions for Project Euler's Problem 1 in Rust

fn main() {
    flexible();
}

// For this solution I focused on flexibility rather than efficiency...
// You can check as many multiples as needed by modifying the array
fn flexible() {
    // Problem Parameters
    let upper_bound: u32 = 1000; // The upper bound of the numbers to check
    let numbers:[u32;2] = [3,5]; // Can update this array to whatever positive numbers are needed

    let mut total: u32 = 0;

    // Check every number from 1 to the upper bound then add it to the total
    for i in 1..upper_bound {
        // Check if the number is a multiple
        for number in numbers.iter() {
            if i % number == 0 { 
                total += i;
                break; // Don't count multiples twice
            }
        }
    }

    println!("The sum of all multiples of {:?} below {} is {}", numbers, upper_bound, total);
}

// Attempting to solve the problem more efficiently
fn efficient() {

}
