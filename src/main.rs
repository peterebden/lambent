use std::path::PathBuf;
extern crate stderrlog;
extern crate structopt;
use structopt::StructOpt;
use lambent;

#[derive(Debug, StructOpt)]
#[structopt(name = "lambent", about = "A utility for managing RGB keyboard backlights.")]
struct Opts {
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
    #[structopt(subcommand)]
    cmd: Command
}

#[derive(Debug, StructOpt)]
enum Command {
    Battery {
        #[structopt(short = "b", long = "battery_file", default_value = "/sys/class/power_supply/BAT0/capacity", parse(from_os_str))]
        battery_file: PathBuf
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
        Command::Battery{ battery_file } => lambent::battery(battery_file.as_path())
    }
}
