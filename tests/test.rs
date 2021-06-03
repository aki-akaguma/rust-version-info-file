mod test_0 {
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
            r#"rustc 1.52.1 (9bc8c42bb 2021-05-09)
rust-version-info-file v0.1.2
"#
        }
    }
    macro_rules! rvi_out_aki_gsub {
        () => {
            r#"rustc 1.52.1 (9bc8c42bb 2021-05-09)
aki-gsub v0.1.29
├── anyhow v1.0.40
├── atty v0.2.14
│   └── libc v0.2.95
├── flood-tide v0.2.2
├── regex v1.5.4
│   ├── aho-corasick v0.7.18
│   │   └── memchr v2.4.0
│   ├── memchr v2.4.0
│   └── regex-syntax v0.6.25
└── runnel v0.3.6
    [build-dependencies]
    └── rustc_version v0.3.3
        └── semver v0.11.0
            └── semver-parser v0.10.2
                └── pest v2.1.3
                    └── ucd-trie v0.1.3
[build-dependencies]
├── rust-version-info-file v0.1.1
└── rustc_version v0.3.3 (*)
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
        assert_eq!(out_s, rvi_out_myself!());
    }
    //
    #[test]
    fn test_aki_gsub() {
        let out_path = "target/rust-version-info-aki-gsub.txt";
        rust_version_info_file(out_path, "fixtures/aki-gsub/Cargo.toml");
        //
        let out_s = read_file(out_path);
        assert_eq!(out_s, rvi_out_aki_gsub!());
    }
}
