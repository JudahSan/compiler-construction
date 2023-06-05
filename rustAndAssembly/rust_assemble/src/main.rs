mod binary_adder;
// mod lib;
mod binary_adder_test;
use binary_adder::*;

fn main() {
    let a = 87;
    let b = 42;

    let sum = binary_adder(a, b);

    println!("{} {} = {}", a, b, sum);
}
