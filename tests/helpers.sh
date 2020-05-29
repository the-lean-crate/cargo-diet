set -e

function in-clone-of () {
  local repo_url=${1:?First argument is the repository URL to clone}
  _context in-clone-of "$repo_url"

  local base_dir=$TMPDIR/cargo-diet-tests
  mkdir -p "$base_dir"

  local repo_name=${repo_url##*/}
  local repo_dir="$base_dir/$repo_name"

  cd $repo_dir 2>/dev/null || {
    cd "$base_dir"
    _note running "${GREEN}" "git clone $repo_url"
    expect_run 0 git clone $repo_url
    cd $repo_name
  }
}