pub fn binary_adder(a: u32, b: u32) -> u32 {
  let mut carry = 0; // Initialize the carry to 0
  let mut result = 0; // Initialize the result to 0

  for i in 0..32 {
    let bit_a = (a >> i) & 1; // Extract the i-th bit of a
    let bit_b = (b >> i) & 1; // Extract the i-th bit of b

    let sum = bit_a ^ bit_b ^ carry; // Calculate the sum of the bits and the carry
    carry = (bit_a & bit_b) | (bit_a & carry) | (bit_b & carry); // Calculate the carry for the next iteration

    result |= sum << i; // Update the result with sum at the i-th position
  }
  result // Return the final result
}

// #[cfg(test)]
// mod tests {
//   use super::binary_adder;

//   #[test]
//   fn test_binary_adder() {
//     // Test case 1
//     let sum = binary_adder(0b101, 0b110);
//     assert_eq!(sum, 0b1011);

//     // Test case 2
//     let sum = binary_adder(0b10101, 0b11010);
//     assert_eq!(sum, 0b11011);

//     // Test case 3
//     let sum = binary_adder(0b1111, 0b1111);
//     assert_eq!(sum, 0b11110);
//   }
// }