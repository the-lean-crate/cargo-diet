### What's new…

#### v1.2.1 - Ignore deleted, unpackable files…

- See https://github.com/the-lean-crate/cargo-diet/issues/7 for details

#### v1.2.0 - Add `--list N` flag…

…to unconditionally show the biggest N entries of the crate at the end of any `cargo diet` operation.

It's best used with `-n` to avoid any actual change to `Cargo.toml`.

#### v1.1.5 - Use more generous exclude globs

* fixes https://github.com/the-lean-crate/cargo-diet/issues/6

#### v1.1.4 - Switch from Structopt/clap to Argh for faster builds and smaller binary

#### v1.1.3 - Support for Rustc 1.42

#### v1.1.2 - Bugfixes

* Fix [off-by-one error](https://github.com/the-lean-crate/cargo-diet/issues/1) when printing the diff
* Improve [error output](https://github.com/the-lean-crate/cargo-diet/issues/2) when `cargo package` fails

#### v1.1.1

* Reduce file-size of produced binaries

#### v1.1.0

* Add the --package-size-limit flag to allow exiting with a non-zero code if the estimated
  crate package size exceeds the given value. This is particularly useful on CI, which can
  leverage the pre-built binaries to get quick access to `cargo diet` without building it
  from source.

#### v1.0.0

Initial release
