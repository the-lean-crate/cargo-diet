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

function remove_paths() {
    sed 's_`/.*`_<redacted>_g'
}

(sandbox
  (with "with no cargo project"
    it "fails with an error message" && {
      SNAPSHOT_FILTER=remove_paths \
      WITH_SNAPSHOT="$snapshot/failure-no-cargo-manifest" \
      expect_run ${WITH_FAILURE} "$exe" diet
    }
  )
  (when "asking for help"
    it "succeeds" && {
      expect_run ${SUCCESSFULLY} "$exe" diet --help
    }
  )
)

(sandbox
  export CARGO_NAME=author
  export CARGO_EMAIL=author@example.com
  
  (with "a newly initialized cargo project"
    step "init cargo project" &&
      expect_run ${SUCCESSFULLY} cargo init --name library --bin

    it "runs successfully" && {
      WITH_SNAPSHOT="$snapshot/success-include-directive-in-new-project" \
      expect_run ${SUCCESSFULLY} "$exe" diet
    }

    it "modifies Cargo.toml to contain an include directive" && {
      expect_snapshot "$snapshot/success-include-directive-in-new-project-cargo-toml" "Cargo.toml"
    }

    (when "running it again" &&
      step "(run it one more time)" && expect_run ${SUCCESSFULLY} "$exe" diet

      it "produces exactly the same output as before" && {
        expect_snapshot "$snapshot/success-include-directive-in-new-project-cargo-toml" "Cargo.toml"
      }
    )
  )
)
