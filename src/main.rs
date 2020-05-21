#[macro_use]
extern crate structopt;

use anyhow::Context;
use structopt::StructOpt;

mod options {
    use std::path::PathBuf;

    #[derive(Debug, StructOpt)]
    #[structopt(name = "example", about = "An example of StructOpt usage.")]
    pub struct Args {
        /// Activate debug mode
        #[structopt(short = "d", long = "debug")]
        pub debug: bool,
        /// Set speed
        #[structopt(short = "s", long = "speed", default_value = "42")]
        pub speed: f64,
        /// Input file
        #[structopt(parse(from_os_str))]
        pub input: PathBuf,
        /// Output file, stdout if not present
        #[structopt(parse(from_os_str))]
        pub output: Option<PathBuf>,
    }
}

fn main() -> anyhow::Result<()> {
    let opt = options::Args::from_args();
    println!("{:?}", opt);
    let path = opt.output.unwrap_or("foo.txt".into());
    cargo_diet::fun(&path).with_context(|| format!("Could not handle file at path {}", path.display()))
}
