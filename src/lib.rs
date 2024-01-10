pub mod registers;
pub mod simulator;
pub mod word;

#[cfg(test)]
mod tests {

    use super::simulator::simulator_init;

    #[test]
    fn running_file() {
        let result = simulator_init();
        assert!(result);
    }
}
