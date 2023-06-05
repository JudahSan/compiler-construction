mod full_adder;

// mod full_adder_test;
mod full_adder_test;
use full_adder::*;

fn main() {

    // Create an instance of FullAdder
    let mut full_adder = FullAdder::new();

    // Set input values
    full_adder.a = true;
    full_adder.b = false;
    full_adder.c = true;

    // Compute the sum and carry
    full_adder.compute();

    // Print the results

    println!("Sum: {}", full_adder.sum);
    println!("Carry: {}", full_adder.carry);
}
