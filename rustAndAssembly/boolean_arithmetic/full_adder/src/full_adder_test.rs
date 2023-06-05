#[cfg(test)]
mod tests {
    use crate::full_adder::FullAdder;

    #[test]
    fn test_full_adder() {
        // Test case 1: Inputs (a=0, b=0, c=0)
        let mut full_adder = FullAdder::new();
        full_adder.compute();
        assert_eq!(full_adder.sum, false);
        assert_eq!(full_adder.carry, false);

        // Test case 2: Inputs (a=1, b=1, c=0)
        let mut full_adder = FullAdder::new();
        full_adder.a = true;
        full_adder.b = true;
        full_adder.compute();
        assert_eq!(full_adder.sum, false);
        assert_eq!(full_adder.carry, true);

        // Test case 3: Inputs (a=1, b=1, c=1)
        let mut full_adder = FullAdder::new();
        full_adder.a = true;
        full_adder.b = true;
        full_adder.c = true;
        full_adder.compute();
        assert_eq!(full_adder.sum, true);
        assert_eq!(full_adder.carry, true);
    }
}
