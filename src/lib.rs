use std::path::Path;
#[macro_use]
extern crate log;

pub fn battery(battery_file: &Path) {
    error!("battery {}", battery_file.display());
}
