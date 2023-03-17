# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 1.2.5 (2023-03-17)

Update dependencies in this maintenance release.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 336 calendar days.
 - 337 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update changelog prior to relrease ([`6ed2215`](https://github.com/the-lean-crate/cargo-diet/commit/6ed22157fb456f9ce7a3992492f16fe6cdca7ac9))
    - Upgrade rmp-serde ([`0d03342`](https://github.com/the-lean-crate/cargo-diet/commit/0d03342df5931c2881abdd0ce2f257dced3601aa))
    - Update dependencies ([`1039553`](https://github.com/the-lean-crate/cargo-diet/commit/1039553dcb8060ca40537094e414cfa5a85633e4))
    - Fix journey tests ([`90f6bda`](https://github.com/the-lean-crate/cargo-diet/commit/90f6bda3b40a3d6fd88ff181b3cb98dba7272fac))
</details>

## 1.2.4 (2022-04-14)

### Bug Fixes

 - <csr-id-132307a032acf5aab67e33cfe7c632e01860e376/> assure cyclic symlinks in package won't break it.
   For now we just ignore any IO error due to limitations in error
   granularity, which might mean that the package size is a little off.
   
   That's preferable over not working at all.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 170 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-diet v1.2.4 ([`d58f624`](https://github.com/the-lean-crate/cargo-diet/commit/d58f62423b7b6f2ed3b569b3739df2ef893f9ff5))
    - Assure cyclic symlinks in package won't break it. ([`132307a`](https://github.com/the-lean-crate/cargo-diet/commit/132307a032acf5aab67e33cfe7c632e01860e376))
</details>

## 1.2.3 (2021-10-26)

Re-release with no functional changes, but a nicer changelog.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release over the course of 50 calendar days.
 - 50 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-diet v1.2.3 ([`5525517`](https://github.com/the-lean-crate/cargo-diet/commit/5525517d9163618fc6fcbba7ee2e6ea8a66f4116))
    - Update changelog prior to release ([`8615e13`](https://github.com/the-lean-crate/cargo-diet/commit/8615e1361f1b4b67de68572c84a79a8ec1891b77))
    - Rewrite changelog ([`61a8c6a`](https://github.com/the-lean-crate/cargo-diet/commit/61a8c6a11a7f19c853b72f5aa9c35b33f52344bf))
    - Cleanup changelog ([`0f712b9`](https://github.com/the-lean-crate/cargo-diet/commit/0f712b946d18b1840664d1de484c85ee068d589f))
    - A new and resilient way to create github releases if they don't exist ([`117db12`](https://github.com/the-lean-crate/cargo-diet/commit/117db128e8e75684dcb22b44715e3c1635aa64d0))
    - Upgdate snapshots to fix tests ([`b310eca`](https://github.com/the-lean-crate/cargo-diet/commit/b310ecaa2d64638f5358415b27dda9d6d9448fbe))
</details>

## v1.2.2 (2021-09-06)

- Dependency upgrades

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 20 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release cargo-diet v1.2.2 ([`82c077a`](https://github.com/the-lean-crate/cargo-diet/commit/82c077af49bb70b15339da7888199f683e97fc9d))
    - Release cargo-diet v1.2.2 ([`bc26818`](https://github.com/the-lean-crate/cargo-diet/commit/bc2681862315563bb11f408c60f5f2456a3b9361))
    - Prepare release ([`8470898`](https://github.com/the-lean-crate/cargo-diet/commit/84708983e3308a13467a2ccfc82bc391ff5f2c38))
    - Dependency upgrades ([`131f55a`](https://github.com/the-lean-crate/cargo-diet/commit/131f55a4f1a40501111f5879cf77016809cc9cc2))
    - Dependency update ([`0625140`](https://github.com/the-lean-crate/cargo-diet/commit/0625140cca53d088f55df593a2e0f013246779b4))
</details>

## v1.2.1 (2021-08-16)

- Ignore deleted, unpackable files…
- See https://github.com/the-lean-crate/cargo-diet/issues/7 for details

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 216 calendar days.
 - 386 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Hopefully get binary releases back ([`be7bb28`](https://github.com/the-lean-crate/cargo-diet/commit/be7bb2897236cbab8287b193252a4c844c88930e))
    - Update snapshots to match reality ([`b79b0f1`](https://github.com/the-lean-crate/cargo-diet/commit/b79b0f15db6c5de4b37e01f7b8d2c08376a9f10e))
    - Release cargo-diet v1.2.1 ([`e64a536`](https://github.com/the-lean-crate/cargo-diet/commit/e64a53650f6b34a7fd7861cd3668d9a88ab1bdaa))
    - Update dependencies ([`3751c2c`](https://github.com/the-lean-crate/cargo-diet/commit/3751c2c5f7b8cde392373920de17fe48a7f17911))
    - Fix #7 ([`51ed634`](https://github.com/the-lean-crate/cargo-diet/commit/51ed6345ffcd22ac5720a64b4de5123f02b37549))
    - Run actions on main ([`eb36e82`](https://github.com/the-lean-crate/cargo-diet/commit/eb36e822de47da92bba7432abe1562a53803a2ae))
    - Fix compile warning ([`abca3ea`](https://github.com/the-lean-crate/cargo-diet/commit/abca3ea180ca92ceb5662fa454fcaaf4527db542))
    - More descriptive header in table of removed files ([`6e1c80d`](https://github.com/the-lean-crate/cargo-diet/commit/6e1c80d7604be66f8245e1084c9c647f0a18cc3d))
</details>

## v1.2.0 (2020-07-26)

Add `--list N` flag to unconditionally show the biggest N entries of the crate at the end of any `cargo diet` operation.

It's best used with `-n` to avoid any actual change to `Cargo.toml`.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump minor version ([`5e00eb8`](https://github.com/the-lean-crate/cargo-diet/commit/5e00eb8522c217a9846e7a6e716e7b0b917bb98b))
    - Add --list flag ([`55b9b49`](https://github.com/the-lean-crate/cargo-diet/commit/55b9b498b7b05a3bd1f8d7ec492a7d9bd9c7b0fc))
</details>

## v1.1.5 (2020-07-25)

Use more generous exclude globs.

* fixes https://github.com/the-lean-crate/cargo-diet/issues/6

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release over the course of 12 calendar days.
 - 22 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump patch level ([`e1e642e`](https://github.com/the-lean-crate/cargo-diet/commit/e1e642ed95dc40801754c8232f66e2b84b8ae3be))
    - Don't optimize build dependencies for lower compile times ([`a05bfa3`](https://github.com/the-lean-crate/cargo-diet/commit/a05bfa323ab689d33f2f98fb319b375219b57a7c))
</details>

## v1.1.4 (2020-07-02)

- Switch from Structopt/clap to Argh for faster builds and smaller binary

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 9 commits contributed to the release over the course of 31 calendar days.
 - 32 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump patch level ([`f0220bb`](https://github.com/the-lean-crate/cargo-diet/commit/f0220bb77ea3a80c80ef3cf6bf0ad919f2bfbd16))
    - Manually implement version argument ([`b397254`](https://github.com/the-lean-crate/cargo-diet/commit/b39725480dd618d7b136490054f4f12e15a156b4))
    - Remove structopt ([`f6c049c`](https://github.com/the-lean-crate/cargo-diet/commit/f6c049c9c8d70757fec11a09a5ceb0adad3828ab))
    - Add argh equivalent for comamnd-line parsing ([`de6b646`](https://github.com/the-lean-crate/cargo-diet/commit/de6b64698be8ab008d8b86c3028a95cc4f725d8e))
    - Update toml_edit ([`15425df`](https://github.com/the-lean-crate/cargo-diet/commit/15425df23a09403b5c40abe3c7b4dad11cfaf84a))
    - Update dependencies ([`471c690`](https://github.com/the-lean-crate/cargo-diet/commit/471c690977e67a3c6815ec301aed73e8b2b5f3fd))
    - Run cargo-fmt ([`03fec6e`](https://github.com/the-lean-crate/cargo-diet/commit/03fec6efedb9e8b456d87e476910cb0e374ba1c5))
    - Create CONTRIBUTING.md ([`39696a7`](https://github.com/the-lean-crate/cargo-diet/commit/39696a7d58cee6be6b0500fce4de402cc5a77773))
    - Add new 'dev-support' feature to allow writing packages for unit-testing ([`3a5f928`](https://github.com/the-lean-crate/cargo-diet/commit/3a5f928ae1a812b346dc0480542e6747f4a433eb))
</details>

## v1.1.3 (2020-05-31)

- Support for Rustc 1.42

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump patch level ([`56ec24f`](https://github.com/the-lean-crate/cargo-diet/commit/56ec24fa2d24a16c1d1bd648607fa895ec3e021f))
    - Don't use a rustc 1.43 feature just yet ([`a5304b1`](https://github.com/the-lean-crate/cargo-diet/commit/a5304b11327184e984243d8f26ccc3c36037dfa0))
</details>

## v1.1.2 (2020-05-29)

* Fix [off-by-one error](https://github.com/the-lean-crate/cargo-diet/issues/1) when printing the diff
* Improve [error output](https://github.com/the-lean-crate/cargo-diet/issues/2) when `cargo package` fails

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 6 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump patch leve ([`c58dba3`](https://github.com/the-lean-crate/cargo-diet/commit/c58dba340c979d722d84a856af6299dd21ab8785))
    - Improve error output and utility in case packaging fails ([`db8a23b`](https://github.com/the-lean-crate/cargo-diet/commit/db8a23b2d909c7e295443aa89bb94b6b6a2ae0e1))
    - Provide human-readable error message if cargo package fails ([`0149e36`](https://github.com/the-lean-crate/cargo-diet/commit/0149e361bd8c527fcdbbe656a9305b9fede7e4ba))
    - Simple fallback for TMPDIR to fix CI ([`d7a07af`](https://github.com/the-lean-crate/cargo-diet/commit/d7a07af40b1e987f1e65114db6731c0f9b856174))
    - Fix off-by-one error :D ([`28734b3`](https://github.com/the-lean-crate/cargo-diet/commit/28734b37e4b6c911b07ff82a535bda0e4bd818e8))
    - Actually opt-level 's' seems to be even better than 'z' ([`9e37afb`](https://github.com/the-lean-crate/cargo-diet/commit/9e37afb23a9d03abcfe26fab5c28b1792baa704a))
</details>

## v1.1.1 (2020-05-28)

* Reduce file-size of produced binaries

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Reduce release binary size; bump patch level ([`e731254`](https://github.com/the-lean-crate/cargo-diet/commit/e731254c567189e1af32aa3971ea4d012b29f76c))
    - Add a note about checking for package size ([`945cd4f`](https://github.com/the-lean-crate/cargo-diet/commit/945cd4f1663b65c47dc56b2f18d1b9bd098f0285))
    - Allow install action to be the first to help it not fail the installation ([`bfd6c41`](https://github.com/the-lean-crate/cargo-diet/commit/bfd6c4115c6fa5dac80af067163a9b9336b6911b))
    - Use actions-rs/install in yet another size validation step ([`6c02844`](https://github.com/the-lean-crate/cargo-diet/commit/6c02844c8d7d44bca214659a74f8d67123fa964f))
    - Add CI exmaple to README file ([`87326d3`](https://github.com/the-lean-crate/cargo-diet/commit/87326d372dd410014d7d4f83ad3d7f50c055992e))
    - Actually invoke the installed binary! ([`e82e103`](https://github.com/the-lean-crate/cargo-diet/commit/e82e1037bbea6d3dc1efe20bba84d4aa678a1609))
    - Use binary release for checking package size limit as well ([`968183e`](https://github.com/the-lean-crate/cargo-diet/commit/968183e1ca62d86d183c44597d62d1f49550959e))
</details>

## v1.1.0 (2020-05-28)

* Add the --package-size-limit flag to allow exiting with a non-zero code if the estimated
  crate package size exceeds the given value. This is particularly useful on CI, which can
  leverage the pre-built binaries to get quick access to `cargo diet` without building it
  from source.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump minor version ([`313cbdd`](https://github.com/the-lean-crate/cargo-diet/commit/313cbdd386362bc52a90431568d355e86c94b9a2))
    - Update documentation ([`5026bf8`](https://github.com/the-lean-crate/cargo-diet/commit/5026bf8ad29ba4033ce7f0f2b8bec2f1fd38cd92))
    - Cargo clippy ([`24fbe20`](https://github.com/the-lean-crate/cargo-diet/commit/24fbe2074af96829e344d73b3332f369e3c98ef1))
    - Cleanup journey tests ([`12ca9a7`](https://github.com/the-lean-crate/cargo-diet/commit/12ca9a7624538f92bc7fd561b8fb85fadd5f02cb))
    - Make journey tests 'crossplatform'… ([`7db4443`](https://github.com/the-lean-crate/cargo-diet/commit/7db44431251387fac324b2ad2be065cc82abb01f))
    - Be a bit more restrictive about the biggest permissive crate size ([`08ba029`](https://github.com/the-lean-crate/cargo-diet/commit/08ba029d3c6f83e02ba0f99131244007844ea0f8))
    - Run itself on CI; print information if package size is not exceeded ([`e9cdff1`](https://github.com/the-lean-crate/cargo-diet/commit/e9cdff19b1a1587379d151007c6b8ac56aa42069))
    - Add support for --package-size-limit flag ([`4859a4d`](https://github.com/the-lean-crate/cargo-diet/commit/4859a4ddef4b57e901e27f61ad43b496f34058e4))
    - Add installation instructions using script ([`c2366a5`](https://github.com/the-lean-crate/cargo-diet/commit/c2366a51478fbb9a68eafc0bdc38cb690cacbb3d))
    - Add adjusted installation script to deal with changed archive structure ([`f39ec30`](https://github.com/the-lean-crate/cargo-diet/commit/f39ec3025ccd0eb3d505989edf0e7b0541507157))
</details>

## v1.0.0 (2020-05-27)

Initial release

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 46 commits contributed to the release over the course of 6 calendar days.
 - 70 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Complete the README ([`333bd34`](https://github.com/the-lean-crate/cargo-diet/commit/333bd34574bd5e264ac63e215a504906fca5c536))
    - Add usage description ([`addb1da`](https://github.com/the-lean-crate/cargo-diet/commit/addb1daf540474470e467bccbde3bf47e234eb41))
    - Add changelog ([`9e82c18`](https://github.com/the-lean-crate/cargo-diet/commit/9e82c18232aaa2510c2216f34d423e262c2c00e4))
    - Don't try to patch things (copied from dua) ([`d018532`](https://github.com/the-lean-crate/cargo-diet/commit/d018532ec1da2fc6b8da2c7d7c2b389d30196837))
    - Try release ([`4446ce1`](https://github.com/the-lean-crate/cargo-diet/commit/4446ce1440e5648a82f9028c1656ad069fe3b6cd))
    - Upgrade Cargo.toml; first pieces of the Readme ([`fc17a7e`](https://github.com/the-lean-crate/cargo-diet/commit/fc17a7ea91df9c92701af883349dbe05660a41ab))
    - Update dependencies ([`ecfc867`](https://github.com/the-lean-crate/cargo-diet/commit/ecfc8670147579145d091bae84d18eb1ea749f43))
    - Always inform about the changes made, without dry-run ([`0658ff6`](https://github.com/the-lean-crate/cargo-diet/commit/0658ff61283a8debae41828a0a09a264799f7dc7))
    - More information about how much we saved ([`d864d94`](https://github.com/the-lean-crate/cargo-diet/commit/d864d94a64ee270daeb407520dc2c8faf18fd039))
    - Don't use total-size-in-bytes - it's not really what we need :D ([`c5e5cdd`](https://github.com/the-lean-crate/cargo-diet/commit/c5e5cdd980ae13bcaae9b21d23abf4189691cc3f))
    - Optimize release build ([`460f896`](https://github.com/the-lean-crate/cargo-diet/commit/460f896ced2fcf0e938c8f887a62557d675f5965))
    - Plan next steps ([`26e7afc`](https://github.com/the-lean-crate/cargo-diet/commit/26e7afc3bda5be2b6e4c761d2f0f91d35e9c2fb0))
    - Prettier diffing ([`36e6f3d`](https://github.com/the-lean-crate/cargo-diet/commit/36e6f3da5fe7778367032effcda1933764166bdb))
    - Correct line skipping computation ([`98f60b3`](https://github.com/the-lean-crate/cargo-diet/commit/98f60b314713df0c24cba3d1f348e638bcabc7e9))
    - Print table instead of plain files ([`b584691`](https://github.com/the-lean-crate/cargo-diet/commit/b58469108daf780ab7b0fac1d4bef10a2a57ff8d))
    - A little nicer dry-run mode printing ([`60c77b4`](https://github.com/the-lean-crate/cargo-diet/commit/60c77b48577f3270cc44d07ca51af49a2a1e1b10))
    - Refactor ([`c9674f3`](https://github.com/the-lean-crate/cargo-diet/commit/c9674f3c0cb09e8afe190c97b7a802f754a2be89))
    - Improved context printing (before and after)… ([`cdc6d72`](https://github.com/the-lean-crate/cargo-diet/commit/cdc6d7251f19f60a5548172c5b920a0715812879))
    - Fixed context before difference ([`9e0e919`](https://github.com/the-lean-crate/cargo-diet/commit/9e0e919fcef303f6e96ccffa31e6381bd8cb61fd))
    - Working context printing ([`8c8fb13`](https://github.com/the-lean-crate/cargo-diet/commit/8c8fb13ddea6d9bd3359d711a3c0931edc3df4ff))
    - Nicer, more simple, line-wise diff formatting ([`5c12ab5`](https://github.com/the-lean-crate/cargo-diet/commit/5c12ab54bcd5441c133f9b7d3b41b2ab06971bb7))
    - Another test that makes clear the diffing doesn't look as intended ([`23b4617`](https://github.com/the-lean-crate/cargo-diet/commit/23b46171743745abf0229a45a9b2716c294dae24))
    - Test to see what happens in dry-run and no change ([`2183504`](https://github.com/the-lean-crate/cargo-diet/commit/218350458ca6ec120c4f8450f74f3d41aa21559d))
    - Support for omitting color based on whether we have a terminal ([`256fc23`](https://github.com/the-lean-crate/cargo-diet/commit/256fc236ab1f2341bd9aca39ef1384cdf4899fc0))
    - Initial version of support for formatting changesets/differences ([`55189a0`](https://github.com/the-lean-crate/cargo-diet/commit/55189a0ee8c9d475a229e8421ed108c1eff253b8))
    - Refactor ([`c98262b`](https://github.com/the-lean-crate/cargo-diet/commit/c98262bba9de06ae85df07f0a694009afdb72670))
    - Very early dry-run implementation ([`68ed21a`](https://github.com/the-lean-crate/cargo-diet/commit/68ed21a16e3ebbf2bcdfa38ead198a1812de77f4))
    - Add --reset flag ([`1b8624d`](https://github.com/the-lean-crate/cargo-diet/commit/1b8624db22a2353a7f29d18d95c83e1cdea3205e))
    - More tests ([`9ca19fd`](https://github.com/the-lean-crate/cargo-diet/commit/9ca19fd6b7153a1fbfec7817fc7a2d03aa907565))
    - Add task list; optimize includes ([`cbf82c4`](https://github.com/the-lean-crate/cargo-diet/commit/cbf82c45f43a7536f22da5fdf8094cc7e7d609d2))
    - Fix previously botched include of cargo-diet itself ([`b10af51`](https://github.com/the-lean-crate/cargo-diet/commit/b10af5181365f55a8053898cea68536be7448674))
    - Need to add fake tar root for algorithm to work properly, neat ([`84b6c73`](https://github.com/the-lean-crate/cargo-diet/commit/84b6c73ea0166f4d1c3ecd9ac01bbdf24e3cac7b))
    - Make journey tests pass, even though they highlight a bug… ([`393db79`](https://github.com/the-lean-crate/cargo-diet/commit/393db7916f0d014c83f29ad58e790540a7a062b8))
    - Initial version of actual 'diet' implementation ([`2408692`](https://github.com/the-lean-crate/cargo-diet/commit/2408692e702da49a503fb7251cf0956f36867650))
    - Add criner-waste-report as engine driving the program ([`b31f50d`](https://github.com/the-lean-crate/cargo-diet/commit/b31f50d287472ac2f20dd02df631f6c701e05e3a))
    - Refactor ([`6911356`](https://github.com/the-lean-crate/cargo-diet/commit/691135660415f1ce6e33f9544519072f74893df6))
    - Make 'cargo init' independent of the environment ([`7254ba9`](https://github.com/the-lean-crate/cargo-diet/commit/7254ba9f46c5a462f280b9b9914663b52e5b1cf5))
    - Fix sed invocation to work similarly on linux ([`cd0aa5a`](https://github.com/the-lean-crate/cargo-diet/commit/cd0aa5aacbcd4650ff0e8f5dad62cd9a08d321dd))
    - Try again with '-E' so that sed on linux works similar to MacOS ([`c60627e`](https://github.com/the-lean-crate/cargo-diet/commit/c60627e1f4fc4683fd1dc9e65a1038b4b20cabcd))
    - Editing works perfectly, thanks toml_edit! ([`7b9e104`](https://github.com/the-lean-crate/cargo-diet/commit/7b9e104d5c217be60f19f4ef0c0d4c5e91c0ebf7))
    - Locate manifest using official means; next: edit toml file ([`2e07d7e`](https://github.com/the-lean-crate/cargo-diet/commit/2e07d7e476d29bbf2a98b4b697c18c5503d07c95))
    - First minimal test cases which already cover the main idea ([`9f62908`](https://github.com/the-lean-crate/cargo-diet/commit/9f62908e96283ebd4bc347b56f872dd5b18139f5))
    - Add status badge ([`2dd21aa`](https://github.com/the-lean-crate/cargo-diet/commit/2dd21aae7b255b7f9a92e84c2e655ee82314f493))
    - Add actions ([`5a2cdf3`](https://github.com/the-lean-crate/cargo-diet/commit/5a2cdf35ec28c5598daeb756426cecc681839a0b))
    - Update to latest error handling ([`b6fa394`](https://github.com/the-lean-crate/cargo-diet/commit/b6fa394ff4321bb53829d4958e3b98acb75f141c))
    - Initial setup - need to get rid of old-school error handling though ;) ([`796d589`](https://github.com/the-lean-crate/cargo-diet/commit/796d589e077fc14471b0563294be6054aa48fdc6))
</details>

## v0.0.0 (2020-03-18)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Allow to publish ([`342b153`](https://github.com/the-lean-crate/cargo-diet/commit/342b15383d97d174f5a0cc9a9c064841b3cc7667))
    - Initial commit ([`18eae12`](https://github.com/the-lean-crate/cargo-diet/commit/18eae12662ff6fdf02e09e388147e7764843cb2d))
</details>

