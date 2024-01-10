pub mod exceptions;
pub mod registers;
pub mod simulator;
pub mod word;

#[cfg(test)]
mod tests {

    use super::registers::Registers;
    use super::simulator::simulator_init;
    use super::word::Word;

    #[test]
    fn test_registers() {
        let mut registers = Registers::new();
        assert!(registers.get("R1").is_ok(), "Cannot access register R1");
        assert!(
            registers.get("YEET").is_err(),
            "Error not thrown while accessing YEET"
        );
        assert!(
            registers.set("R1", Word::Integer { value: 47 }).is_ok(),
            "Cannot set R1 to 47"
        );
        assert!(
            registers
                .set(
                    "YEET",
                    Word::String {
                        value: String::from("YEET")
                    }
                )
                .is_err(),
            "Error not thrown while accessing YEET"
        );

        assert!(
            registers
                .get("R1")
                .is_ok_and(|x| if let Word::Integer { value } = x {
                    value == 47
                } else {
                    false
                }),
            "Register R1 contains value is that is different from the value set"
        );
    }

    #[test]
    fn initialise_simulator() {
        let result = simulator_init();
        assert!(result.is_ok());
    }
}
