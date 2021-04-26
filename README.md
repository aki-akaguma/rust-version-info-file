# rust-version-info-file

*rust-version-info-file* is generating one file includes rustc version and cargo tree.

## Easy to use

Write following into The build.rs:

```rust
use rust_version_info_file::rust_version_info_file;

fn main() {
    rust_version_info_file("target/rust-version-info.txt");
}
```

## On debian package

In Cargo.toml

```text
[package.metadata.deb]
assets = [
    ["target/rust-version-info.txt", "usr/share/doc/your_package/", "644"],
    ["README.md", "usr/share/doc/your_package/", "644"],
]
```
