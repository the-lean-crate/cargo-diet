#[macro_use]
extern crate structopt;

use structopt::StructOpt;

mod args {
    #[derive(Debug, StructOpt)]
    #[structopt(bin_name = "cargo")]
    pub enum Command {
        /// Add an include directive minimizing bandwidth usage when downloading from crates.io
        #[structopt(name = "diet")]
        #[structopt(
            after_help = "This command allows you to add an include directive to minimize the package size of your crate."
        )]
        Diet(Args),
    }

    #[derive(Debug, StructOpt)]
    pub struct Args {
        #[structopt(
            long,
            short = "r",
            help = "If set, existing includes and excludes will be removed prior to running the command.\
        That way, new files outside of any included directory will be picked up."
        )]
        pub reset: bool,
    }
}

fn main() -> anyhow::Result<()> {
    let args::Command::Diet(args) = args::Command::from_args();
    cargo_diet::execute(cargo_diet::Options { reset: args.reset })?;
    Ok(())
}
