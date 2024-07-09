use {chrono::Local, std::{thread, fmt::{Display, Formatter, Result}}};

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

pub fn log_string(status: LogStatus, text: String) {
    log_str(status, text.as_str());
}

pub fn log_str(status: LogStatus, text: &str) {
    println!("({0}) [{1}] {2}: {3}", get_current_time(), status, get_current_thread_name(), text);
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