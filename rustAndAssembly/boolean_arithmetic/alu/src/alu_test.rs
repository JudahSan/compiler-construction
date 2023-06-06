#[cfg(test)]
mod tests {
    // use super::alu::ALU;
    use crate::alu::ALU;

    #[test]
    fn test_alu() {
        // Test case 1: Addition
        let mut alu = ALU::new();
        alu.x = [false; 16];
        alu.y = [false; 16];
        alu.zx = true;
        alu.nx = false;
        alu.zy = true;
        alu.ny = false;
        alu.f = true;
        alu.no = false;
        alu.compute();
        assert_eq!(alu.out, [false; 16]);
        assert_eq!(alu.zr, true);
        assert_eq!(alu.ng, false);

        // Test case 2: Bitwise AND
        let mut alu = ALU::new();
        alu.x = [false, false, true, true, false, false, true, true, false, false, true, true, false, false, true, true];
        alu.y = [true, true, false, false, true, true, false, false, true, true, false, false, true, true, false, false];
        alu.zx = false;
        alu.nx = false;
        alu.zy = false;
        alu.ny = false;
        alu.f = false;
        alu.no = false;
        alu.compute();
        assert_eq!(alu.out, [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]);
        assert_eq!(alu.zr, true);
        assert_eq!(alu.ng, false);
    }

    // Add more test cases as needed
}
