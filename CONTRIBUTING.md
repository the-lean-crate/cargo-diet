Thank you for considering to contribute!
All contributions are welcome, may it be in the form of issues and PRs.

To get you started on adjusting code, it's important to know that this program is split into three parts:

1. frontend for Cargo
   * a program invoked like `cargo diet` supporting a few flags
1. A library
   * glue between the `criner-waste-report` crate and the command-line arguments
   * editing of Cargo.toml files
   * pretty-printing of changes and savings
1. The [`criner-waste-report`](https://github.com/the-lean-crate/criner/tree/master/criner-waste-report) crate
   * The actual logic to determine which include and/or excludes to provide
  
For `1)` and `2)` the best way to validate changes in code is to run `make continuous-journey-tests`, which are defined in [this shell script](https://github.com/the-lean-crate/cargo-diet/blob/3a5f928ae1a812b346dc0480542e6747f4a433eb/tests/stateless-journey.sh#L35).
`cargo diet` is tested on the highest possible level, only validating functionality in the way it is actually seen by the user.
The tests are by no means complete, but cover common combinations of flags and prevent regressions or unanticipated changes in
the most common scenarios.

For `3)`, it's best to clone [criner](https://github.com/crates-io/criner) and `cd criner-waste-report`. From there unit tests
are run with `cargo test`. The respective [binary fixtures](https://github.com/the-lean-crate/criner/tree/master/criner-waste-report/tests/fixtures)
can be generated conveniently using the _dev-support_ feature `cargo build --features dev-support`, which allows it to be called
on any sample crate such as in `/path/to/cargo-diet/target/debug/cargo-diet diet -n --save-package-for-unit-test /path/to/criner/criner-waste-report/mycrate-v1.rmp`.
Now a new assertion [similar to this one](https://github.com/the-lean-crate/criner/blob/master/criner-waste-report/src/test/from_package.rs#L18)
can be added, allowing to state a new expectation and fast iterations on the code without risking to break anything in unexpected ways.

If there are questions or the need for suggestions, we are happy to help in the respective issues or PRs.
