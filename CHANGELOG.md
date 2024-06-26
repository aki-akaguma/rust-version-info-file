# Changelog: rust-version-info-file

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


## [0.1.10] (2024-06-09)
### Changed
* test and build support 1.65.0 on github workflows

### Fixed
* test: illegal result string

## [0.1.9] (2023-02-12)
### Removed
* `COPYING`

### Changed
* refactored `Makefile`

### Fixed
* `LICENSE-APACHE`, `LICENSE-MIT`

## [0.1.8] (2023-01-31)
### Added
* `.github/workflows/test-ubuntu.yml`
* `.github/workflows/test-macos.yml`
* `.github/workflows/test-windows.yml`
* test status badges into `README.tpl`
* `rust-version = "1.58.0"` into `Cargo.toml`

### Fixed
* license files
* clippy: `bool_assert_comparison`
* bug: test result FAILED on windows

## [0.1.7] (2023-01-10)
### Added
* badges into `README.md`

### Changed
* reformat `CHANGELOG.md`
* update crates: regex(1.7)

### Fixed
* bug: test_0::test_aki_gsub is failed
* clippy: uninlined_format_args

## [0.1.6] (2022-06-12)
### Changed
* changes to edition 2021
* updates: fixture/aki-gsub

## [0.1.5] (2022-05-21)
### Fixed
* bug: test_0::test_aki_gsub

## [0.1.4] (2022-02-08)
### Fixed
* bug: tests.

## [0.1.3] (2021-11-14)
### Added
* more documents.

### Fixed
* test: test match with regex.

## [0.1.2] (2021-06-03)
### Added
* parameter: cargo_toml_file into rust_version_info_file()

### Changed
* some change output format

## [0.1.1] (2021-04-26)
### Fixed
* bug: thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', /home/xxx/.cargo/registry/src/github.com-1ecc6299db9ec823/rust-version-info-file-0.1.0/src/lib.rs:15:14

## [0.1.0] (2021-04-22)
* first commit

[Unreleased]: https://github.com/aki-akaguma/rust-version-info-file/compare/v0.1.10..HEAD
[0.1.10]: https://github.com/aki-akaguma/rust-version-info-file/compare/v0.1.9..v0.1.10
[0.1.9]: https://github.com/aki-akaguma/rust-version-info-file/compare/v0.1.8..v0.1.9
[0.1.8]: https://github.com/aki-akaguma/rust-version-info-file/compare/v0.1.7..v0.1.8
[0.1.7]: https://github.com/aki-akaguma/rust-version-info-file/compare/v0.1.6..v0.1.7
[0.1.6]: https://github.com/aki-akaguma/rust-version-info-file/compare/v0.1.5..v0.1.6
[0.1.5]: https://github.com/aki-akaguma/rust-version-info-file/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/aki-akaguma/rust-version-info-file/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/rust-version-info-file/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/rust-version-info-file/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/rust-version-info-file/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/rust-version-info-file/releases/tag/v0.1.0
