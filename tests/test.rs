mod test_0 {
    use regex::Regex;
    use std::path::Path;
    use rust_version_info_file::rust_version_info_file;
    //
    fn read_file<T: AsRef<Path>>(src: T) -> String {
        use std::io::Read;
        let src_path: &Path = src.as_ref();
        match std::fs::OpenOptions::new()
            .create(false)
            .read(true)
            .open(src_path)
        {
            Ok(mut fi) => {
                let mut s = String::new();
                let _ = fi.read_to_string(&mut s);
                s
            }
            Err(_) => String::new(),
        }
    }
    //
    macro_rules! rvi_out_myself {
        () => {
            r#"rustc \d+\.\d+\.\d+(-nightly|-beta\.[^ ]+)? \([0-9a-z]{9} \d{4}-\d{2}-\d{2}\)
rust-version-info-file v\d+\.\d+\.\d+
\[dev-dependencies\]
└── regex v\d+\.\d+\.\d+
    ├── aho-corasick v\d+\.\d+\.\d+
    │   └── memchr v\d+\.\d+\.\d+
    ├── memchr v\d+\.\d+\.\d+
    └── regex-syntax v\d+\.\d+\.\d+
"#
        }
    }
    macro_rules! rvi_out_aki_gsub {
        () => {
            r#"rustc \d+\.\d+\.\d+(-nightly|-beta\.[^ ]+)? \([0-9a-z]{9} \d{4}-\d{2}-\d{2}\)
aki-gsub v0\.1\.34
├── anyhow v1\.\d+\.\d+
├── atty v0\.\d+\.\d+
│   └── libc v0\.\d+\.\d+
├── flood-tide v0\.\d+\.\d+
├── memx-cdy v0\.\d+\.\d+
│   ├── libc v0\.\d+\.\d+
│   └── memx v0\.\d+\.\d+
│       └── cpufeatures v0\.\d+\.\d+
│       \[build-dependencies\]
│       └── rustc_version v0\.\d+\.\d+
│           └── semver v1\.\d+\.\d+
├── regex v1\.\d+\.\d+
│   ├── aho-corasick v0\.\d+\.\d+
│   │   └── memchr v2\.\d+\.\d+
│   ├── memchr v2\.\d+\.\d+
│   └── regex-syntax v0\.\d+\.\d+
└── runnel v0\.\d+\.\d+
    \[build-dependencies\]
    └── rustc_version v0\.\d+\.\d+ \(\*\)
\[build-dependencies\]
├── rust-version-info-file v0\.\d+\.\d+
└── rustc_version v0\.\d+\.\d+ \(\*\)
\[dev-dependencies\]
├── exec-target v0\.\d+\.\d+
└── indoc v1\.\d+\.\d+ \(proc-macro\)
"#
        }
    }
    //
    #[test]
    fn test_myself() {
        let out_path = "target/rust-version-info-myself.txt";
        rust_version_info_file(out_path, "Cargo.toml");
        //
        let out_s = read_file(out_path);
        let re = Regex::new(rvi_out_myself!()).unwrap();
        assert_eq!(re.is_match(&out_s), true);
    }
    //
    #[test]
    fn test_aki_gsub() {
        let out_path = "target/rust-version-info-aki-gsub.txt";
        rust_version_info_file(out_path, "fixtures/aki-gsub/Cargo.toml");
        //
        let out_s = read_file(out_path);
        let re = Regex::new(rvi_out_aki_gsub!()).unwrap();
        assert_eq!(re.is_match(&out_s), true);
    }
}
