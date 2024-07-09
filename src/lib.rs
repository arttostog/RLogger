use {chrono::Local, std::{fmt::{Display, Formatter, Result}, thread}};
mod logging_to_file;
mod colored_log;

pub enum LogStatus {
    INFO,
    WARN,
    ERROR
}

impl Display for LogStatus {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match  *self {
            LogStatus::INFO => write!(formatter, "INFO"),
            LogStatus::WARN => write!(formatter, "WARN"),
            LogStatus::ERROR => write!(formatter, "ERROR")
        }
    }
}

pub fn enable_logging_to_file(path: &str, remove_last_log: bool) {
    logging_to_file::enable_logging_to_file(path, remove_last_log);
}

pub fn enable_colors_in_console() {
    colored_log::enable_colors_in_console();
}

pub fn log_str(status: LogStatus, text: &str) {
    let log: String = format!("({0}) [{1}] {2}: {3}", get_current_time(), status, get_current_thread_name(), text);
    
    match logging_to_file::print_log_to_file(log.clone()) {
        Ok(_) => (),
        Err(error) => panic!("Error appending to file: {}", error)
    };
    
    colored_log::print_log(status, log);
}

fn get_current_thread_name() -> String {
    String::from(match thread::current().name() {
        Some(name) => name,
        None => "undefined_thread"
    })
}

fn get_current_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}