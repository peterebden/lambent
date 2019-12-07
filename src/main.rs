use std::path::PathBuf;
#[macro_use]
extern crate log;
extern crate stderrlog;
#[macro_use]
extern crate structopt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "lambent", about = "A utility for managing RGB keyboard backlights.")]
struct Opts {
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
}


fn main() {
    let opt = Opt::from_args();
    stderrlog::new()
        .module(module_path!())
        .verbosity(opt.verbose)
        .init()
        .unwrap();
    info!("Hello, world");
}
