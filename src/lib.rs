pub mod simulator;

#[cfg(test)]
mod tests {

    use super::simulator::simulator_init;

    #[test]
    fn running_file() {
        let result = simulator_init();
        assert!(result);
    }
}
