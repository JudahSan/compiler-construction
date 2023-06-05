mod half_adder;

mod half_adder_test;

use half_adder::HalfAdder;

fn main() {
    let mut half_adder = HalfAdder::new(false, true);
    half_adder.compute();

    println!("Sum: {}", half_adder.sum);
    println!("Carry: {}", half_adder.carry);
}
