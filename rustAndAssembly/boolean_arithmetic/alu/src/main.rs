mod alu;
mod alu_test;
fn main() {
    // Create an instance of the ALU
    let mut alu = alu::ALU::new();

    // Set the input values and control signals
    alu.x = [false, false, true, true, false, false, true, true, false, false, true, true, false, false, true, true];
    alu.y = [true, true, false, false, true, true, false, false, true, true, false, false, true, true, false, false];
    alu.zx = false;
    alu.nx = false;
    alu.zy = false;
    alu.ny = false;
    alu.f = false;
    alu.no = false;

    // Compute the result
    alu.compute();

    // Print the output
    println!("ALU Output: {:?}", alu.out);
    println!("Zero Flag (ZR): {}", alu.zr);
    println!("Negative Flag (NG): {}", alu.ng);
}
