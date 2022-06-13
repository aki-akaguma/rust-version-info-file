# rust-version-info-file

output rust version info into a file

This crate is the presents, the file output of rustc --version and cargo tree command.

## Features

- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

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
rustc 1.61.0 (fe5b13d68 2022-05-18)
aki-gsub v0.1.34
├── anyhow v1.0.57
├── atty v0.2.14
│   └── libc v0.2.126
├── flood-tide v0.2.4
├── memx-cdy v0.1.7
│   ├── libc v0.2.126
│   └── memx v0.1.20
│       [build-dependencies]
│       └── rustc_version v0.4.0
│           └── semver v1.0.10
├── regex v1.5.6
│   ├── aho-corasick v0.7.18
│   │   └── memchr v2.5.0
│   ├── memchr v2.5.0
│   └── regex-syntax v0.6.26
└── runnel v0.3.10
    [build-dependencies]
    └── rustc_version v0.4.0 (*)
[build-dependencies]
├── rust-version-info-file v0.1.5
└── rustc_version v0.4.0 (*)
[dev-dependencies]
├── exec-target v0.2.6
└── indoc v1.0.6 (proc-macro)
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
