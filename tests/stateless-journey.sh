#!/usr/bin/env bash
set -eu

exe=${1:?First argument must be the executable to test}

root="$(cd "${0%/*}" && pwd)"
exe="$(cd "${exe%/*}" && echo "$(pwd)/${exe##*/}")"
# shellcheck disable=1090
source "$root/utilities.sh"
snapshot="$root/snapshots"

SUCCESSFULLY=0
WITH_FAILURE=1

(sandbox
  (with "with no cargo project"
    it "fails with an error message" && {
      WITH_SNAPSHOT="$snapshot/failure-no-cargo-manifest" \
      expect_run ${WITH_FAILURE} "$exe"
    }
  )
  (when "asking for help"
    it "succeeds" && {
      expect_run ${SUCCESSFULLY} "$exe" --help
    }
  )
)

(sandbox
  (with "a newly initialized cargo project"
    step "init cargo project" &&
      expect_run ${SUCCESSFULLY} cargo init --name library --bin

    it "runs successfully" && {
      WITH_SNAPSHOT="$snapshot/success-include-directive-in-new-project" \
      expect_run ${SUCCESSFULLY} "$exe"
    }

    it "modifies Cargo.toml to contain an include directive" && {
      expect_snapshot "$snapshot/success-include-directive-in-new-project-cargo-toml" "Cargo.toml"
    }

    when "running it again" && expect_run ${SUCCESSFULLY} "$exe"

    it "produces exactly the same output as before" && {
      expect_snapshot "$snapshot/success-include-directive-in-new-project-cargo-toml" "Cargo.toml"
    }
  )
)
