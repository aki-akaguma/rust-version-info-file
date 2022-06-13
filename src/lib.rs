#![allow(clippy::needless_doctest_main)]
/*!
output rust version info into a file

This crate is the presents, the file output of rustc --version and cargo tree command.

# Features

- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

# Examples

Please write the following code in the build.rs:

```rust
use rust_version_info_file::rust_version_info_file;

fn main() {
    rust_version_info_file("target/rust-version-info.txt", "Cargo.toml");
}
```

And you get the file as result it.

```text
cat target/rust-version-info-file.txt
```

# On debian package

In Cargo.toml

```text
[package.metadata.deb]
assets = [
    ["target/rust-version-info.txt", "usr/share/doc/your_package/", "644"],
    ["README.md", "usr/share/doc/your_package/", "644"],
]
```

# Output sample

```text
$ cat target/rust-version-info-aki-gsub.txt
```

```text
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
*/
use std::io::{Read, Write};
use std::path::Path;
use std::fs::OpenOptions;

/// output rust version info into a file
pub fn rust_version_info_file<T: AsRef<Path>>(dst: T, cargo_toml_file: &str) {
    let dst_path: &Path = dst.as_ref();
    let old_s = read_file(dst_path);
    let curr_s = format!("{}\n{}\n", rustc_version_info(), tree_version_info(cargo_toml_file),);
    if old_s != curr_s {
        let mut fo = match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(dst_path) {
            Ok(fo) => fo,
            Err(_) => return,
        };
        let _ = fo.write_fmt(format_args!("{}", curr_s));
        let _ = fo.flush();
    }
}

fn read_file(path: &Path) -> String {
    match OpenOptions::new()
        .create(false)
        .read(true)
        .open(path)
    {
        Ok(mut fi) => {
            let mut s = String::new();
            let _ = fi.read_to_string(&mut s);
            s
        }
        Err(_) => String::new(),
    }
}

fn rustc_version_info() -> String {
    let cmd = std::env::var_os("RUSTC").unwrap_or_else(|| std::ffi::OsString::from("rustc"));
    let out = std::process::Command::new(cmd)
        .arg("--version")
        .output()
        .unwrap();
    let v = out.stdout;
    let v_len = v.len();
    let v = if v_len > 0 && v[v_len - 1] == b'\n' {
        &v[0..v_len - 1]
    } else {
        &v[0..v_len]
    };
    String::from_utf8(v.to_vec()).unwrap()
}

fn tree_version_info(cargo_toml_file: &str) -> String {
    let cmd = "cargo";
    let out = std::process::Command::new(cmd)
        .arg("tree")
        .arg("--manifest-path")
        .arg(cargo_toml_file)
        .output()
        .unwrap();
    let v = out.stdout;
    let v_len = v.len();
    let v = if v_len > 0 && v[v_len - 1] == b'\n' {
        &v[0..v_len - 1]
    } else {
        &v[0..v_len]
    };
    //
    let string = String::from_utf8(v.to_vec()).unwrap();
    //
    let string = match string.find(" (") {
        Some(pos1) => {
            let (s1, s2) = string.split_at(pos1);
            match s2.find(')') {
                Some(pos2) => {
                    let (_s2, s3) = s2.split_at(pos2+1);
                    s1.to_owned() + s3
                }
                None => string,
            }
        },
        None => string,
    };
    //
    string
}
