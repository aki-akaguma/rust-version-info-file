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
aki-gsub v0\.1\.29
├── anyhow v1\.0\.45
├── atty v0\.2\.14
│   └── libc v0\.2\.107
├── flood-tide v0\.2\.3
├── regex v1\.5\.4
│   ├── aho-corasick v0\.7\.18
│   │   └── memchr v2\.4\.1
│   ├── memchr v2\.4\.1
│   └── regex-syntax v0\.6\.25
└── runnel v0\.3\.8
    \[build-dependencies\]
    └── rustc_version v0\.4\.0
        └── semver v1\.0\.4
\[build-dependencies\]
├── rust-version-info-file v0\.1\.2
└── rustc_version v0\.3\.3
    └── semver v0\.11\.0
        └── semver-parser v0\.10\.2
            └── pest v2\.1\.3
                └── ucd-trie v0\.1\.3
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
