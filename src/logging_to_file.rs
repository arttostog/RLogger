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