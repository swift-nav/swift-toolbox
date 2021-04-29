#[cfg(feature = "tests")]
mod log_test_impl {

    use log::Level;
    use logtest::Logger;

    use console_backend::utils::compute_doppler;

    #[test]
    fn log_test() {
        // Start the logger.

        let mut logger = Logger::start();

        compute_doppler(
            123438650.3359375,
            123438590.203125,
            251746.8,
            251746.8,
            false,
        );

        let log_msg = logger.pop().unwrap();
        assert_eq!(
            log_msg.args(),
            "Received two complete observation sets with identical TOW"
        );
        assert_eq!(log_msg.level(), Level::Warn);
    }
}
