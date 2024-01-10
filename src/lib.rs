pub mod exceptions;
pub mod registers;
pub mod simulator;
pub mod word;

#[cfg(test)]
mod tests {

    use super::simulator::simulator_init;

    #[test]
    fn initialise_simulator() {
        let result = simulator_init();
        assert!(result.is_ok());
    }
}
