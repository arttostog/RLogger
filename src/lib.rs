pub mod logger {
    use {chrono::Local, std::{fmt::{Display, Formatter, Result}, thread}};

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
        logger_to_file::enable_logging_to_file(path, remove_last_log);
    }

    pub fn log_str(status: LogStatus, text: &str) {
        let log: String = format!("({0}) [{1}] {2}: {3}", get_current_time(), status, get_current_thread_name(), text);
        
        println!("{}", log);
        
        match logger_to_file::print_log_to_file(log) {
            Ok(_) => (),
            Err(error) => panic!("Error appending to file: {}", error)
        };
    }

    fn get_current_thread_name() -> String {
        String::from(match thread::current().name() {
            Some(name) => name,
            None => "Undefined_thread"
        })
    }

    fn get_current_time() -> String {
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }

    mod logger_to_file {
        use std::{fs::{metadata, remove_file, File, OpenOptions}, io::{Error, Write}};

        static mut PATH_TO_LOGGING: String = String::new();

        pub fn enable_logging_to_file(path: &str, remove_last_log: bool) {
            unsafe {
                PATH_TO_LOGGING = path.to_string();

                if remove_last_log {
                    let path: &str = PATH_TO_LOGGING.as_str();
                    if metadata(path).is_ok() {
                        match remove_file(path) {
                            Ok(_) => (),
                            Err(error) => panic!("Error removing file: {}", error)
                        };
                    }
                }
            }
        }

        pub fn print_log_to_file(log: String) -> Result<(), Error> {
            unsafe {
                if PATH_TO_LOGGING != String::new() {
                    let mut file: File = OpenOptions::new().create(true).append(true).open(PATH_TO_LOGGING.as_str())?;
                    return file.write_all(format!("{}\n", log).as_bytes());
                }
            }
            Ok(())
        }
    }
}