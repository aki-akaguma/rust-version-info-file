# rust-version-info-file

output rust version info file

This crate is the presents, the file output of rustc --version and cargo tree command.

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


License: MIT OR Apache-2.0
