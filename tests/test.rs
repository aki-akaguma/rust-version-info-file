mod test_0 {
    use regex::{Regex, RegexBuilder};
    use rust_version_info_file::rust_version_info_file;
    use std::path::Path;
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
                #[cfg(windows)]
                let s = s.replace("\r\n", "\n");
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
    ├── regex-automata v\d+\.\d+\.\d+
    │   ├── aho-corasick v\d+\.\d+\.\d+ \(\*\)
    │   ├── memchr v\d+\.\d+\.\d+
    │   └── regex-syntax v\d+\.\d+\.\d+
    └── regex-syntax v\d+\.\d+\.\d+
"#
        };
    }
    macro_rules! rvi_out_aki_gsub {
        () => {
            r#"rustc \d+\.\d+\.\d+(-nightly|-beta\.[^ ]+)? \([0-9a-z]{9} \d{4}-\d{2}-\d{2}\)
aki-gsub v0\.1\.34
├── anyhow v\d+\.\d+\.\d+
├── atty v\d+\.\d+\.\d+
│   └── (libc|winapi) v\d+\.\d+\.\d+
├── flood-tide v\d+\.\d+\.\d+
├── memx-cdy v\d+\.\d+\.\d+
│   ├── libc v\d+\.\d+\.\d+
│   └── memx v\d+\.\d+\.\d+
│       └── cpufeatures v\d+\.\d+\.\d+(
│           └── libc v\d+\.\d+\.\d+)?
├── regex v\d+\.\d+\.\d+
│   ├── aho-corasick v\d+\.\d+\.\d+
│   │   └── memchr v\d+\.\d+\.\d+
│   ├── memchr v\d+\.\d+\.\d+
│   ├── regex-automata v\d+\.\d+\.\d+
│   │   ├── aho-corasick v\d+\.\d+\.\d+ \(\*\)
│   │   ├── memchr v\d+\.\d+\.\d+
│   │   └── regex-syntax v\d+\.\d+\.\d+
│   └── regex-syntax v\d+\.\d+\.\d+
└── runnel v\d+\.\d+\.\d+
    \[build-dependencies\]
    └── rustc_version v\d+\.\d+\.\d+
        └── semver v\d+\.\d+\.\d+
\[build-dependencies\]
├── rust-version-info-file v\d+\.\d+\.\d+
└── rustc_version v\d+\.\d+\.\d+ \(\*\)
\[dev-dependencies\]
├── exec-target v\d+\.\d+\.\d+
└── indoc v\d+\.\d+\.\d+ \(proc-macro\)
"#
        };
    }
    //
    #[test]
    fn test_myself() {
        let out_path = "target/rust-version-info-myself.txt";
        rust_version_info_file(out_path, "Cargo.toml");
        //
        let out_s = read_file(out_path);
        let re = Regex::new(rvi_out_myself!()).unwrap();
        assert!(re.is_match(&out_s));
    }
    //
    #[test]
    fn test_aki_gsub() {
        let out_path = "target/rust-version-info-aki-gsub.txt";
        rust_version_info_file(out_path, "fixtures/aki-gsub/Cargo.toml");
        //
        let out_s = read_file(out_path);
        //let re = Regex::new(rvi_out_aki_gsub!()).unwrap();
        let re = RegexBuilder::new(rvi_out_aki_gsub!())
            .multi_line(true)
            .build()
            .unwrap();
        if !re.is_match(&out_s) {
            assert_eq!(&out_s, "");
        }
        assert!(re.is_match(&out_s));
    }
}
