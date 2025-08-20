# rust-version-info-file

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

output rust version info into a file

This crate is the presents, the file output of rustc --version and cargo tree command.

## Features

- minimum support rustc 1.65.0 (897e37553 2022-11-02)

## Examples

Please write the following code in the build.rs:

```rust
use rust_version_info_file::rust_version_info_file;

fn main() {
    rust_version_info_file("target/rust-version-info.txt", "Cargo.toml");
}
```

And you get the file as result it.

```
cat target/rust-version-info-file.txt
```

## On debian package

In Cargo.toml

```
[package.metadata.deb]
assets = [
    ["target/rust-version-info.txt", "usr/share/doc/your_package/", "644"],
    ["README.md", "usr/share/doc/your_package/", "644"],
]
```

## Output sample

```
$ cat target/rust-version-info-aki-gsub.txt
```

```
rustc 1.89.0 (29483883e 2025-08-04)
aki-gsub v0.1.34
├── anyhow v1.0.99
├── atty v0.2.14
│   └── libc v0.2.175
├── flood-tide v0.2.11
├── memx-cdy v0.1.13
│   ├── libc v0.2.175
│   └── memx v0.1.32
│       └── cpufeatures v0.2.17
├── regex v1.11.1
│   ├── aho-corasick v1.1.3
│   │   └── memchr v2.7.5
│   ├── memchr v2.7.5
│   ├── regex-automata v0.4.9
│   │   ├── aho-corasick v1.1.3 (*)
│   │   ├── memchr v2.7.5
│   │   └── regex-syntax v0.8.5
│   └── regex-syntax v0.8.5
└── runnel v0.3.19
[build-dependencies]
├── rust-version-info-file v0.1.10
└── rustc_version v0.4.1
    └── semver v1.0.26
[dev-dependencies]
├── exec-target v0.2.9
└── indoc v1.0.9 (proc-macro)
```

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/rust-version-info-file/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/rust-version-info-file.svg
[crate-link]: https://crates.io/crates/rust-version-info-file
[docs-image]: https://docs.rs/rust-version-info-file/badge.svg
[docs-link]: https://docs.rs/rust-version-info-file/
[rustc-image]: https://img.shields.io/badge/rustc-1.65+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/rust-version-info-file/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/rust-version-info-file/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/rust-version-info-file/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/rust-version-info-file/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/rust-version-info-file/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/rust-version-info-file/actions/workflows/test-windows.yml
