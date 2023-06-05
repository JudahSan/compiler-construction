mod adder;
mod adder_test;

use adder::Adder;

fn main() {
    // Create an instance of the Adder 
    let mut adder = Adder::new();

    // Set the input values
    adder.a = true;
    adder.b = true;

    // Compute the sum

    adder.compute();

    // Print the sum and carry

    println!("Sum: {}", adder.sum);
    println!("Carry: {}", adder.carry);

}

