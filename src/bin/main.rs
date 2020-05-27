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
        #[structopt(long, short = "r")]
        /// If set, existing include and exclude directives will be removed prior to running the command.
        ///
        /// That way, new files outside of any included directory will be picked up."
        pub reset_manifest: bool,

        #[structopt(long, short = "n")]
        /// If set, no change will actually be made to the Cargo.toml file, simulating what would be done instead.
        pub dry_run: bool,
    }
}

use args::{Args, Command};

fn main() -> anyhow::Result<()> {
    let Command::Diet(Args {
        reset_manifest: reset,
        dry_run,
    }) = Command::from_args();
    cargo_diet::execute(
        cargo_diet::Options {
            reset,
            dry_run,
            colored_output: atty::is(atty::Stream::Stdout),
        },
        std::io::stdout().lock(),
    )?;
    Ok(())
}
