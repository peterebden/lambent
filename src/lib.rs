use std::path::Path;
use std::fs;
use std::thread;
use std::time::Duration;
#[macro_use]
extern crate log;

pub fn battery(battery_file: &Path, kbd_file: &Path, sleep_duration: Duration) {
    let mut current_capacity = -5;
    loop {
        let contents = match fs::read_to_string(battery_file) {
            Ok(contents) => contents,
            Err(error) => {
                panic!("Error reading file: {}", error);
            }
        };
        let capacity = contents.trim().parse::<i32>().unwrap();
        info!("Current capacity: {}", capacity);
        if capacity != current_capacity {
            update_brightness(kbd_file, capacity);
            current_capacity = capacity;
        }
        thread::sleep(sleep_duration);
    }
}

fn update_brightness(kbd_file: &Path, capacity: i32) {

}
