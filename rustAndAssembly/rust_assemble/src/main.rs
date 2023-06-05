mod binary_adder;
use binary_adder::binary_adder;

fn main() {
    let a = 87;
    let b = 42;

    let sum = binary_adder(a, b);

    println!("{} {} = {}", a, b, sum);
}
