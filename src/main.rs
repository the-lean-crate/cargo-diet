#[macro_use]
extern crate structopt;

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
        /// Output file, stdout if not present
        #[structopt(parse(from_os_str))]
        pub output: Option<PathBuf>,
    }
}

fn main() -> anyhow::Result<()> {
    let opt = options::Args::from_args();
    println!("{:?}", opt);
    cargo_diet::fun()?;
    Ok(())
}
