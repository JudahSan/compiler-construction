#[cfg(test)]
mod tests {
    use crate::binary_adder;
    use assert_eq;

    #[test]
    fn test_binary_adder() {
        // Test case 1
        let sum = binary_adder(0b101, 0b110);
        assert_eq!(sum, 0b1011);

        // Test case 2
        let sum = binary_adder(0b10101, 0b11010);
        assert_eq!(sum, 0b101111);

        // Test case 3
        let sum = binary_adder(0b1111, 0b1111);
        assert_eq!(sum, 0b11110); // Corrected expected output
    }
}
