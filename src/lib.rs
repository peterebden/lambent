use std::path::Path;
use std::fs;
use std::time::Duration;
use std::time::Instant;
use hsl::HSL;
use signal::trap::Trap;
use signal::Signal;
#[macro_use]
extern crate log;

pub fn battery(battery_file: &Path, kbd_file: &Path, sleep_duration: Duration) {
    let trap = Trap::trap(&[Signal::SIGHUP]);
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
            update_keyboard(kbd_file, capacity);
            current_capacity = capacity;
        }
        trap.wait(Instant::now().checked_add(sleep_duration).unwrap());
    }
}

fn update_keyboard(kbd_file: &Path, capacity: i32) {
    let green = HSL::from_rgb(&[0, 255, 0]);
    let red = HSL::from_rgb(&[255, 0, 0]);
    let blue = HSL::from_rgb(&[0, 0, 255]);
    if capacity < 10 {
        update_keyboard_colour(kbd_file, &blue);
    } else if capacity < 20 {
        update_keyboard_colour(kbd_file, &red);
    } else {
        let proportion = (capacity - 20) as f64 / 80.0;
        let h = proportion * (green.h - red.h) + red.h;
        update_keyboard_colour(kbd_file, &HSL{h: h, s: red.s, l: red.l});
    }
}

fn update_keyboard_colour(kbd_file: &Path, colour: &HSL) {
    let (r, g, b) = colour.to_rgb();
    info!("new rgb: {},{},{}", r, g, b);
    match fs::write(kbd_file, [r, g, b]) {
        Ok(()) => {},
        Err(error) => error!("Failed to update keyboard backlight: {}", error),
    }
}
