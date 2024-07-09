use colored::Colorize;
use super::LogStatus;

pub static mut ENABLE_COLORS: bool = false;

pub fn enable_colors_in_console() {
    unsafe {
        ENABLE_COLORS = true;
    }
}

pub fn print_log(status: LogStatus, log: String) {
    unsafe {
        if ENABLE_COLORS {
            println!("{}", match status {
                LogStatus::INFO => log,
                LogStatus::WARN => log.yellow().to_string(),
                LogStatus::ERROR => log.red().to_string()
            });
            return;
        }
    }
    println!("{}", log);
}