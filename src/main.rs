use std::path::PathBuf;
extern crate stderrlog;
extern crate structopt;
use humantime::Duration;
use structopt::StructOpt;
use lambent;

#[derive(Debug, StructOpt)]
#[structopt(name = "lambent", about = "A utility for managing RGB keyboard backlights.")]
struct Opts {
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Battery {
        #[structopt(short = "b", long = "battery_file", default_value = "/sys/class/power_supply/BAT0/capacity", parse(from_os_str))]
        battery_file: PathBuf,
        #[structopt(short = "k", long = "kbd_file", default_value = "/sys/bus/hid/drivers/razerkbd/0003:1532:0239.0004/matrix_effect_static", parse(from_os_str))]
        kbd_file: PathBuf,
        #[structopt(short = "d", long = "sleep_duration", default_value = "30s")]
        sleep_duration: Duration,
    },
}

fn main() {
    let opts = Opts::from_args();
    stderrlog::new()
        .module(module_path!())
        .verbosity(opts.verbose)
        .init()
        .unwrap();
    match opts.cmd {
        Command::Battery{ battery_file, kbd_file, sleep_duration } => lambent::battery(battery_file.as_path(), kbd_file.as_path(), *sleep_duration)
    }
}
