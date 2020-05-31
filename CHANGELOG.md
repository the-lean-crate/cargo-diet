### What's new…

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
