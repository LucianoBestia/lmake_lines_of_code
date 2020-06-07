//! utilsmod.rs

//use lazy_static::lazy_static;
//use regex::Regex;
use unwrap::unwrap;

use std::{fs, io, path::Path};
/// traverse dir (sub-dir) with exclude dir
/// the find_file and the exclude dir strings must start with /
pub fn traverse_dir_with_exclude_dir(
    dir: &Path,
    find_file: &str,
    exclude_dirs: &Vec<String>,
) -> io::Result<Vec<String>> {
    // if the parameter is /*.rs, I can eliminate /*
    let find_file = &find_file.replace("/*", "");

    let mut v = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let str_path = unwrap!(path.to_str());
            if path.is_dir() {
                let mut is_excluded = false;
                for excl in exclude_dirs {
                    if str_path.ends_with(excl) {
                        is_excluded = true;
                        break;
                    }
                }
                if !is_excluded {
                    let mut sub_v = traverse_dir_with_exclude_dir(&path, find_file, exclude_dirs)?;
                    v.append(&mut sub_v);
                }
            } else {
                if str_path.ends_with(find_file) {
                    v.push(str_path.to_string());
                }
            }
        }
    }
    Ok(v)
}
