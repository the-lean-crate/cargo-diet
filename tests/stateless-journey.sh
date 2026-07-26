#!/usr/bin/env bash
set -eu

exe=${1:?First argument must be the executable to test}

root="$(cd "${0%/*}" && pwd)"
exe="$(cd "${exe%/*}" && echo "$(pwd)/${exe##*/}")"
# shellcheck disable=1090
source "$root/utilities.sh"
source "$root/helpers.sh"
snapshot="$root/snapshots"

SUCCESSFULLY=0
WITH_FAILURE=1

function remove_paths() {
    sed 's_`/.*`_<redacted>_g'
}

# This filter is required as bytecounts fluctuate due to changing meta-data/timestamps
function remove_bytecounts() {
    sed -E 's/[0-9]+ B/<bytecount>/g'
}


(in-clone-of https://github.com/greshake/i3status-rust
  stepn "reproduce https://github.com/the-lean-crate/cargo-diet/issues/1"
  (when "cargo diet in dry-run mode"
    it "runs successfully" && {
      expect_run ${SUCCESSFULLY} "$exe" diet -n
    }
  )
)

(sandbox
  (with "with no cargo project"
    it "fails with an error message" && {
      SNAPSHOT_FILTER=remove_paths \
      WITH_SNAPSHOT="$snapshot/failure-no-cargo-manifest" \
      expect_run ${WITH_FAILURE} "$exe" diet
    }

    it "fails with the same error message with --quiet" && {
      SNAPSHOT_FILTER=remove_paths \
      WITH_SNAPSHOT="$snapshot/failure-no-cargo-manifest" \
      expect_run ${WITH_FAILURE} "$exe" diet --quiet
    }

    it "fails with the same error message with -q" && {
      SNAPSHOT_FILTER=remove_paths \
      WITH_SNAPSHOT="$snapshot/failure-no-cargo-manifest" \
      expect_run ${WITH_FAILURE} "$exe" diet -q
    }
  )
  (with "with an invalid cargo project"
    echo "foobar" > Cargo.toml
    it "fails with a human readable error message" && {
      SNAPSHOT_FILTER=remove_paths \
      WITH_SNAPSHOT="$snapshot/failure-invalid-cargo-manifest" \
      expect_run ${WITH_FAILURE} "$exe" diet
    }
  )
  (with "with a cargo project that causes package failure"
    cp "$root/fixtures/artichoke-cargo.toml" Cargo.toml
    it "fails with a human readable error message" && {
      SNAPSHOT_FILTER=remove_paths \
      WITH_SNAPSHOT="$snapshot/failure-invalid-cargo-manifest-due-to-packaging-with-human-readable-error" \
      expect_run ${WITH_FAILURE} "$exe" diet
    }
  )
  (when "asking for help"
    it "succeeds" && {
      expect_run ${SUCCESSFULLY} "$exe" diet --help
    }
  )
)

(with "a cargo user and email"
  export CARGO_NAME=author
  export CARGO_EMAIL=author@example.com

  (sandbox
    (with "a newly initialized cargo project with one dependency that doesnt exist"
      step "init cargo project" &&
        expect_run ${SUCCESSFULLY} cargo init --name library --bin
        echo 'should-never-exist-on-cratesio = "42.21.10"' >> Cargo.toml

      it "succeeds" && {
        SNAPSHOT_FILTER=remove_paths \
        WITH_SNAPSHOT="$snapshot/success-invalid-dependencies-but-valid-package" \
        expect_run ${SUCCESSFULLY} "$exe" diet
      }

      (when "running it with --save-package-for-unit-test flag"
        FILE_PATH=name-of-package-for-testing.rmp
        it "succeeds" && {
          WITH_SNAPSHOT="$snapshot/success-save-package" \
          expect_run ${SUCCESSFULLY} "$exe" diet --save-package-for-unit-test "${FILE_PATH}"
        }

        it "creates the file at the given path" && {
         expect_exists "${FILE_PATH}"
        }
      )
    )
  )

  (sandbox
    (with "a newly initialized cargo project"
      step "init cargo project" &&
        expect_run ${SUCCESSFULLY} cargo init --edition 2018 --name library --bin

      (with "the --quiet flag set"
        it "runs successfully without producing output" && {
          WITH_SNAPSHOT="$snapshot/success-quiet" \
          expect_run ${SUCCESSFULLY} "$exe" diet --quiet
        }

        it "runs successfully without producing output with -q" && {
          WITH_SNAPSHOT="$snapshot/success-quiet" \
          expect_run ${SUCCESSFULLY} "$exe" diet -q
        }

        it "still updates Cargo.toml" && {
          expect_snapshot "$snapshot/success-include-directive-in-new-project-cargo-toml" "Cargo.toml"
        }
      )
    )
  )

  (sandbox
    (with "a newly initialized cargo project"
      step "init cargo project" &&
        expect_run ${SUCCESSFULLY} cargo init --edition 2018 --name library --bin

      (with "the --dry-run flag set"
        it "runs successfully and states the crate is lean" && {
          WITH_SNAPSHOT="$snapshot/success-include-directive-in-new-project-dry-run" \
          expect_run ${SUCCESSFULLY} "$exe" diet --dry-run
        }

        it "does not modify the Cargo.toml file" && {
          expect_snapshot "$snapshot/success-include-directive-in-new-project-cargo-toml" "Cargo.toml"
        }
      )

      (with "NO --dry-run flag set"
        it "runs successfully and states the crate is lean" && {
          WITH_SNAPSHOT="$snapshot/success-include-directive-in-new-project" \
          expect_run ${SUCCESSFULLY} "$exe" diet
        }

        it "does not modify the Cargo.toml file" && {
          expect_snapshot "$snapshot/success-include-directive-in-new-project-cargo-toml" "Cargo.toml"
        }
      )

      (when "running it again"
        it "runs successfully" && {
          WITH_SNAPSHOT="$snapshot/success-include-directive-in-new-project" \
          expect_run ${SUCCESSFULLY} "$exe" diet
        }

        it "produces exactly the same output as before" && {
          expect_snapshot "$snapshot/success-include-directive-in-new-project-cargo-toml" "Cargo.toml"
        }
      )

      (with "a new test file which is part of the src/ directory"
        touch src/lib_test.rs

        (with "the -n (dry-run) flag set"
          it "runs successfully and prints diff information" && {
            WITH_SNAPSHOT="$snapshot/success-include-directive-in-new-project-test-added-dry-run" \
            expect_run ${SUCCESSFULLY} "$exe" diet -n
          }

          it "does not alter Cargo.toml" && {
            expect_snapshot "$snapshot/success-include-directive-in-new-project-cargo-toml-with-tests-excluded-dry-run" "Cargo.toml"
          }
        )

        (with "NO --dry-run flag set"
          (when "running it again"
            it "runs successfully and states the changes" && {
              WITH_SNAPSHOT="$snapshot/success-include-directive-in-new-project-test-added" \
              expect_run ${SUCCESSFULLY} "$exe" diet
            }

            it "produces a new include directive which explicitly excludes the new file type" && {
              expect_snapshot "$snapshot/success-include-directive-in-new-project-cargo-toml-with-tests-excluded" "Cargo.toml"
            }
          )
        )
      )
      (with "a new README file in the project root"
        touch README.md

        (when "running it"
          it "runs successfully" && {
            WITH_SNAPSHOT="$snapshot/success-include-directive-in-new-project" \
            expect_run ${SUCCESSFULLY} "$exe" diet
          }

          it "produces the same include as before, effectively not picking up a file it should pick up!" && {
            expect_snapshot "$snapshot/success-include-directive-in-new-project-cargo-toml-with-tests-excluded" "Cargo.toml"
          }
        )

        (with "the --reset-manifest flag set"
          (with "the --dry-run flag set"
            it "runs successfully" && {
              WITH_SNAPSHOT="$snapshot/success-in-new-project-readme-reset-dry-run" \
              expect_run ${SUCCESSFULLY} "$exe" diet --reset-manifest --dry-run
            }

            it "does not alter the Cargo.toml file" && {
              expect_snapshot "$snapshot/success-include-directive-in-new-project-cargo-toml-with-tests-excluded" "Cargo.toml"
            }
          )

          (with "NO --dry-run flag set"
            it "runs successfully" && {
              WITH_SNAPSHOT="$snapshot/success-include-directive-in-new-project-readme-reset-no-dryrun" \
              expect_run ${SUCCESSFULLY} "$exe" diet -r
            }

            it "produces a new include that includes the new file." && {
              expect_snapshot "$snapshot/success-include-directive-in-new-project-cargo-toml-with-tests-excluded-and-readme" "Cargo.toml"
            }
          )
        )

        (with "NO --reset-manifest flag"
          it "runs successfully" && {
            WITH_SNAPSHOT="$snapshot/success-include-directive-in-new-project" \
            expect_run ${SUCCESSFULLY} "$exe" diet
          }

          it "does not alter the cargo manifest" && {
            expect_snapshot "$snapshot/success-include-directive-in-new-project-cargo-toml-with-tests-excluded-and-readme" "Cargo.toml"
          }
        )

        (with "the --package-size-limit flag"
          (when "the limit is lower than the actual package size"
            it "runs successfully" && {
              SNAPSHOT_FILTER=remove_bytecounts \
              WITH_SNAPSHOT="$snapshot/success-include-directive-in-new-project-limit-exceeded" \
              expect_run ${WITH_FAILURE} "$exe" diet --package-size-limit 50B
            }

            it "still prints the error with -q" && {
              SNAPSHOT_FILTER=remove_bytecounts \
              WITH_SNAPSHOT="$snapshot/failure-package-size-limit-exceeded-quiet" \
              expect_run ${WITH_FAILURE} "$exe" diet -q --package-size-limit 50B
            }

            it "does not put a file in target/package" && {
              expect_run ${WITH_FAILURE} find target/package
            }
          )
          (when "the limit is higher than the actual package size"
            it "runs successfully" && {
              SNAPSHOT_FILTER=remove_bytecounts \
              WITH_SNAPSHOT="$snapshot/success-include-directive-in-new-project-limit-not-exceeded" \
              expect_run ${SUCCESSFULLY} "$exe" diet --package-size-limit 50KB
            }

            it "does not put a file in target/package" && {
              expect_run ${WITH_FAILURE} find target/package
            }
          )
        )

        (with "the --list flag"
          (when "a small amount is specified"
            it "runs successfully and lists the given amount of entries entries in the package" && {
              SNAPSHOT_FILTER=remove_bytecounts \
              WITH_SNAPSHOT="$snapshot/success-list-with-limit" \
              expect_run ${SUCCESSFULLY} "$exe" diet -n --list 2
            }

            it "does not put a file in target/package" && {
              expect_run ${WITH_FAILURE} find target/package
            }
          )
          (when "a 0 is specified as amount"
            it "runs successfully and lists all entries entries in the package" && {
              SNAPSHOT_FILTER=remove_bytecounts \
              WITH_SNAPSHOT="$snapshot/success-list-no-limit-with-zero" \
              expect_run ${SUCCESSFULLY} "$exe" diet -n --list 0
            }

            it "does not put a file in target/package" && {
              expect_run ${WITH_FAILURE} find target/package
            }
          )
        )
      )
    )
  )
)

(sandbox
  (with "a cargo workspace whose members are declared via a glob, one of which needs a leaner include directive"
    step "set up workspace" && {
      cat <<EOF > Cargo.toml
[workspace]
members = ["crates/*"]
resolver = "2"
EOF
      workspace_member crates/crate-a crate-a
      workspace_member crates/crate-b crate-b
      touch crates/crate-b/src/lib_test.rs
    }

    (when "running it at the workspace root with no package selection flags"
      it "processes every workspace member, mirroring cargo's own default-members behavior" && {
        WITH_SNAPSHOT="$snapshot/success-workspace-default-members" \
        expect_run ${SUCCESSFULLY} "$exe" diet -n
      }
    )

    (when "running it with -p crate-a"
      it "only processes the selected package" && {
        WITH_SNAPSHOT="$snapshot/success-workspace-dash-p" \
        expect_run ${SUCCESSFULLY} "$exe" diet -n -p crate-a
      }
    )

    (when "running it with -p pointing at a package that does not exist"
      it "fails with a cargo-like error message" && {
        WITH_SNAPSHOT="$snapshot/failure-workspace-unknown-package" \
        expect_run ${WITH_FAILURE} "$exe" diet -n -p does-not-exist
      }
    )

    (when "running it from within a workspace member directory, with no flags"
      it "still only touches that one member, as it did before workspaces were supported" && (
        cd crates/crate-a
        WITH_SNAPSHOT="$snapshot/success-workspace-member-cwd-no-flags" \
        expect_run ${SUCCESSFULLY} "$exe" diet -n
      )
    )

    (when "running it with --save-package-for-unit-test and more than one package selected"
      it "refuses, since that flag can only write a single package description" && {
        WITH_SNAPSHOT="$snapshot/failure-workspace-save-package-with-multiple-targets" \
        expect_run ${WITH_FAILURE} "$exe" diet -n --workspace --save-package-for-unit-test out.rmp
      }
    )

    (when "running it with -p crate-b --workspace together"
      it "lets --workspace silently override -p, just like cargo itself does" && {
        WITH_SNAPSHOT="$snapshot/success-workspace-overrides-dash-p" \
        expect_run ${SUCCESSFULLY} "$exe" diet -n -p crate-b --workspace
      }
    )

    (when "running it with --workspace --exclude crate-b"
      it "processes every member except the excluded one" && {
        WITH_SNAPSHOT="$snapshot/success-workspace-excluding-member" \
        expect_run ${SUCCESSFULLY} "$exe" diet -n --workspace --exclude crate-b
      }
    )

    (when "running it with --exclude but without --workspace"
      it "fails just like cargo itself does in this case" && {
        WITH_SNAPSHOT="$snapshot/failure-workspace-exclude-without-workspace-flag" \
        expect_run ${WITH_FAILURE} "$exe" diet -n --exclude crate-b
      }
    )

    (when "running it with --workspace, excluding every member"
      it "fails instead of silently doing nothing, just like cargo itself does in this case" && {
        WITH_SNAPSHOT="$snapshot/failure-workspace-excludes-everything" \
        expect_run ${WITH_FAILURE} "$exe" diet -n --workspace --exclude crate-a --exclude crate-b
      }
    )

    (when "running it with --workspace and a --package-size-limit that crate-a exceeds"
      it "names the offending package in the error, since with several packages in play that's no longer obvious" && {
        SNAPSHOT_FILTER=remove_bytecounts \
        WITH_SNAPSHOT="$snapshot/failure-workspace-package-size-limit-names-package" \
        expect_run ${WITH_FAILURE} "$exe" diet -n --workspace --package-size-limit 1B
      }
    )
  )
)

(sandbox
  (with "a cargo workspace with explicit member paths and several subcrates, two of which need a leaner include directive"
    step "set up workspace" && {
      cat <<EOF > Cargo.toml
[workspace]
members = ["pkg-one", "pkg-two", "pkg-three"]
resolver = "2"
EOF
      workspace_member pkg-one pkg-one
      touch pkg-one/src/lib_test.rs

      workspace_member pkg-two pkg-two
      touch pkg-two/src/lib_test.rs

      workspace_member pkg-three pkg-three
    }

    (when "running it with --workspace, with NO --dry-run flag set"
      it "runs successfully and reports each package in turn" && {
        WITH_SNAPSHOT="$snapshot/success-workspace-multiple-members-modified" \
        expect_run ${SUCCESSFULLY} "$exe" diet --workspace
      }

      it "adds the include directive to pkg-one's manifest" && {
        expect_snapshot "$snapshot/success-workspace-pkg-one-cargo-toml" "pkg-one/Cargo.toml"
      }

      it "adds the include directive to pkg-two's manifest" && {
        expect_snapshot "$snapshot/success-workspace-pkg-two-cargo-toml" "pkg-two/Cargo.toml"
      }

      it "leaves pkg-three's manifest untouched, since it was already lean" && {
        expect_snapshot "$snapshot/success-workspace-pkg-three-cargo-toml" "pkg-three/Cargo.toml"
      }
    )
  )
)

(sandbox
  (with "a cargo workspace that explicitly declares no default members"
    step "set up workspace" && {
      cat <<EOF > Cargo.toml
[workspace]
members = ["crate-a"]
default-members = []
resolver = "2"
EOF
      workspace_member crate-a crate-a
    }

    (when "running it at the workspace root with no package selection flags"
      it "fails, since cargo itself packages nothing in this case rather than falling back to every member" && {
        WITH_SNAPSHOT="$snapshot/failure-workspace-empty-default-members" \
        expect_run ${WITH_FAILURE} "$exe" diet -n
      }
    )
  )
)

(sandbox
  (with "a newly initialized cargo project with several dependencies"
    step "init cargo project" &&
      expect_run ${SUCCESSFULLY} cargo init --edition 2018 --name library --bin

    printf 'rust = "1"\nis = "1"\na = "1"\nmust = "1"\n' >> Cargo.toml

    (with "a new test file which is part of the src/ directory"
      touch src/lib_test.rs

      (with "the -n (dry-run) flag set"
        it "reports a truncated lines" && {
          WITH_SNAPSHOT="$snapshot/success-include-directive-trailing-context-dry-run" \
          expect_run ${SUCCESSFULLY} "$exe" diet -n
        }
      )
    )
  )
)
