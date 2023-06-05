#[cfg(test)]
mod tests {
    use crate::adder::Adder;

    #[test]
    fn test_adder() {
        // Test case 1: Inputs (a=0, b=0)
        let mut adder = Adder::new();
        adder.compute();
        assert_eq!(adder.sum, false);
        assert_eq!(adder.carry, false);

        // Test case 2: Inputs (a=1, b=0)
        let mut adder = Adder::new();
        adder.a = true;
        adder.compute();
        assert_eq!(adder.sum, true);
        assert_eq!(adder.carry, false);

        // Test case 3: Inputs (a=0, b=1)
        let mut adder = Adder::new();
        adder.b = true;
        adder.compute();
        assert_eq!(adder.sum, true);
        assert_eq!(adder.carry, false);

        // Test case 4: Inputs (a=1, b=1)
        let mut adder = Adder::new();
        adder.a = true;
        adder.b = true;
        adder.compute();
        assert_eq!(adder.sum, false);
        assert_eq!(adder.carry, true);
    }
}
