pub mod logger;

pub fn create_test_log() {
    logger::log_str(logger::LogStatus::INFO, "Test Log");
}