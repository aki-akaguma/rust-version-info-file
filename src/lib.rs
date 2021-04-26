use std::io::{Read, Write};
use std::path::Path;
use std::fs::OpenOptions;

pub fn rust_version_info_file<T: AsRef<Path>>(dst: T) {
    let dst_path: &Path = dst.as_ref();
    let old_s = read_file(dst_path);
    let curr_s = format!("{}\n{}\n", rustc_version_info(), tree_version_info(),);
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

fn tree_version_info() -> String {
    let cmd = "cargo";
    let out = std::process::Command::new(cmd)
        .arg("tree")
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
