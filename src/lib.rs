use std::path::Path;
use std::fs;
use std::thread;
use std::time::Duration;
use hsl::HSL;
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
    let green = HSL::from_rgb(&[0, 255, 0]);
    let red = HSL::from_rgb(&[255, 0, 0]);
    let proportion = capacity as f64 / 100.0;
    let h = proportion * (green.h - red.h) + red.h;  // This assumes green.h > red.h, which is indeed the case.
    info!("New colour for capacity {}: {}", capacity, h);
    let (r, g, b) = HSL{h: h, s: red.s, l: red.l}.to_rgb();
    match fs::write(kbd_file, [r, g, b]) {
        Ok(()) => {},
        Err(error) => error!("Failed to update keyboard backlight: {}", error),
    }
}
