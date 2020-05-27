[![Rust](https://github.com/the-lean-crate/cargo-diet/workflows/Rust/badge.svg)](https://github.com/the-lean-crate/cargo-diet/actions?query=workflow%3ARust)

`cargo diet` is a companion program of [The Lean Crate Initiative][lean-crate-initiative] to help computing 'optimal' `include` directives for your
Cargo.toml manifest. 'optimal' here is the smallest size a crate can have while retaining everything relevant to building the code and populating `crates.io` and
`lib.rs` with useful information on the landing page.

Please note that this opinionated approach won't prevent you to adjust the initial `include` directive to your liking and needs, without `cargo diet`
interfering with you on subsequent invocations.

[![asciicast](https://asciinema.org/a/UKhYox6XXwWgnVSVWm5PIdUf5.svg)](https://asciinema.org/a/UKhYox6XXwWgnVSVWm5PIdUf5)

[lean-crate-initiative]: https://github.com/the-lean-crate/criner
