mod register;
mod register_test;

fn main() {
    // Create an instance of the register
    let mut reg = register::Bit::new();

    // Set input and load values
    reg.set_input(1);
    reg.set_load(1);

    // Update the register
    reg.update();

    // Get the output value
    let output = reg.get_output();  

    // Print the output
    println!("Output: {}", output);
}
