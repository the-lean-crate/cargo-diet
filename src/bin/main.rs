mod args {
    use argh::FromArgs;

    /// Make your crate lean
    #[derive(Debug, FromArgs)]
    #[argh(name = "cargo")]
    pub struct Args {
        /// add an include directive minimizing bandwidth usage when downloading from crates.io
        #[argh(subcommand)]
        pub cmd: Subcommands,
    }

    /// this command allows you to add an include directive to minimize the package size of your crate.
    #[derive(Debug, FromArgs)]
    #[argh(subcommand)]
    pub enum Subcommands {
        Diet(Diet),
    }

    /// this command allows you to add an include directive to minimize the package size of your crate.
    #[derive(Debug, FromArgs)]
    #[argh(subcommand, name = "diet")]
    pub struct Diet {
        #[argh(switch, short = 'v')]
        /// if set, print the program version.
        pub version: bool,

        #[argh(switch, short = 'r')]
        /// if set, existing include and exclude directives will be removed prior to running the command.
        ///
        /// That way, new files outside of any included directory will be picked up."
        pub reset_manifest: bool,

        #[argh(switch, short = 'n')]
        /// if set, no change will actually be made to the Cargo.toml file, simulating what would be done instead.
        pub dry_run: bool,

        #[argh(option, short = 'l')]
        /// provide any number N to limit the output to the N biggest files, ascending.
        ///
        /// if to 0, list all entries that would be in the crate with the current configuration.
        /// Otherwise list the given amount of entries.
        pub list: Option<usize>,

        #[argh(option, from_str_fn(parse_size))]
        /// if set, and the estimated compressed size of the package would exceed the given size, i.e. 40KB, the command
        /// will exit with a non-zero exit code.
        ///
        /// The test is performed based on data from before any change was made to the Cargo.toml file, and doesn't affect
        /// any other flags.
        ///
        /// This is particularly useful when running on CI to avoid allowing the package to become too big unexpectedly, which
        /// can happen if big files are placed in currently included directories.
        pub package_size_limit: Option<u64>,

        #[argh(option)]
        #[cfg(feature = "dev-support")]
        /// if set, this specifies the path at which a package description for use in criner-waste-report tests should be written to.
        pub save_package_for_unit_test: Option<std::path::PathBuf>,
    }

    fn parse_size(src: &str) -> Result<u64, String> {
        byte_unit::Byte::from_str(src)
            .map_err(|err| err.to_string())
            .and_then(|b| {
                if b.get_bytes() > std::u64::MAX as u128 {
                    Err("Value is too large".into())
                } else {
                    Ok(b.get_bytes() as u64)
                }
            })
    }
}

use args::{Args, Diet, Subcommands};

fn main() -> anyhow::Result<()> {
    let cmd = argh::from_env::<Args>().cmd;
    let Subcommands::Diet(Diet {
        version,
        reset_manifest: reset,
        dry_run,
        list,
        package_size_limit,
        #[cfg(feature = "dev-support")]
        save_package_for_unit_test,
    }) = cmd;
    if version {
        println!(env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    cargo_diet::execute(
        cargo_diet::Options {
            reset,
            dry_run,
            colored_output: atty::is(atty::Stream::Stdout),
            list,
            package_size_limit,
            #[cfg(feature = "dev-support")]
            save_package_for_unit_test,
        },
        std::io::stdout().lock(),
    )?;
    Ok(())
}
