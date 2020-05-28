[![Rust](https://github.com/the-lean-crate/cargo-diet/workflows/Rust/badge.svg)](https://github.com/the-lean-crate/cargo-diet/actions?query=workflow%3ARust)
[![Crates.io](https://img.shields.io/crates/v/cargo-diet.svg)](https://crates.io/crates/cargo-diet)

`cargo diet` is a companion program of [The Lean Crate Initiative][lean-crate-initiative] to help computing 'optimal' `include` directives for your
Cargo.toml manifest. 'optimal' here is the smallest size a crate can have while retaining everything relevant to building the code and populating `crates.io` and
`lib.rs` with useful information on the landing page.

Please note that this opinionated approach won't prevent you to adjust the initial `include` directive to your liking and needs, without `cargo diet`
interfering with you on subsequent invocations.

[![asciicast](https://asciinema.org/a/UKhYox6XXwWgnVSVWm5PIdUf5.svg)](https://asciinema.org/a/UKhYox6XXwWgnVSVWm5PIdUf5)

[lean-crate-initiative]: https://github.com/the-lean-crate/criner

### Usage

* **make the crate lean by editing the `Cargo.toml` file in place**
  * `cargo diet`
  
* **simulate how the `Cargo.toml` file would be edited to obtain a lean crate**
  * `cargo diet -n`  or `cargo diet --dry-run`
  
* **force computing an `include` directive even though one exists already**
  * `cargo diet -r` or `cargo diet --reset-manifest`
  * can also be used with `--dry-run` such as in `cargo diet --reset-manifest --dry-run` or `cargo diet -rn`
  
* **prevent the crate from exceeding a certain package size (best on CI)**
  * `cargo diet -n --package-size-limit 50KB`
  * See the installation instructions specifically for CI, allowing to quickly download a pre-built binary
  
  
### Installation

#### Using Cargo

With a recent version of `cargo` (obtainable using [rustup][rustup]), the following should work:

```bash
cargo install cargo-diet
```

[rustup]: https://rustup.rs/

#### Using GitHub releases

Pre-built binaries can be found in the [releases](https://github.com/the-lean-crate/cargo-diet/releases) section of this repository.

You can use an [installation script][install.sh] to automate this process:

```bash 
curl -LSfs https://raw.githubusercontent.com/the-lean-crate/cargo-diet/master/ci/install.sh | \
    sh -s -- --git the-lean-crate/cargo-diet
```

#### On CI for controlling the crate package size

Assuming tests are running on a linux box, the following script will install the latest version of `cargo diet`
and run it with flags to assert a crate package size does not exceed _10KB_, a value that can freely be chosen.

That way it's easy to get a warning if there are new and possibly unexpected files in one of the directories which
are included already.

```bash 
curl -LSfs https://raw.githubusercontent.com/the-lean-crate/cargo-diet/master/ci/install.sh | \
    sh -s -- --git the-lean-crate/cargo-diet --target x86_64-unknown-linux-musl

cargo diet --dry-run --package-size-limit 10KB
```

Also have a look at [how this is actually used][gh-action-usage] in GitHub Actions.

[gh-action-usage]: https://github.com/the-lean-crate/cargo-diet/blob/e82e1037bbea6d3dc1efe20bba84d4aa678a1609/.github/workflows/rust.yml#L23-L26
[install.sh]: https://github.com/the-lean-crate/cargo-diet/blob/master/ci/install.sh

### Development

#### Run tests

```bash
make journey-tests
```

#### Learn about other targets

```
make
```

