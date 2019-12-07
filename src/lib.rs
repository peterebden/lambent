use std::path::Path;
use std::fs;
use std::thread;
use std::time::Duration;
#[macro_use]
extern crate log;

pub fn battery(battery_file: &Path, sleep_duration: Duration) {
    loop {
        let contents = match fs::read_to_string(battery_file) {
            Ok(contents) => contents,
            Err(error) => {
                panic!("Error reading file: {}", error);
            }
        };
        info!("Contents: {}", contents);
        let capacity = contents.trim().parse::<i32>().unwrap();
        info!("Current capacity: {}", capacity);


        thread::sleep(sleep_duration);
    }
}
