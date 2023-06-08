#[cfg(test)]
mod tests {
    use crate::register::Bit;

    #[test]
    fn test_register_update() {
        let mut reg = Bit::new();

        // Test loading input value
        reg.set_input(1);
        reg.set_load(1);
        reg.update();
        assert_eq!(reg.get_output(), 1);

        // Test not loading input value
        reg.set_input(0);
        reg.set_load(0);
        reg.update();
        assert_eq!(reg.get_output(), 1);
    }
}
