use regex::Regex;
use std::{
    env::current_dir,
    fs::{self, read_dir, DirEntry},
};

fn main() {
    let dir = current_dir().unwrap();
    let mut files: Vec<_> = read_dir(dir)
        .unwrap()
        .filter_map(|entry| match entry {
            Ok(entry) => {
                if is_leetcode_file(&entry) {
                    let number: i32 = Regex::new(r"\d+")
                        .unwrap()
                        .captures(entry.file_name().to_str().unwrap())
                        .unwrap()
                        .get(0)
                        .unwrap()
                        .as_str()
                        .parse()
                        .unwrap();
                    Some((number, entry.file_name()))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();
    files.sort_by(|(a, _), (b, _)| a.cmp(b));
    let segments: Vec<_> = files
        .iter()
        .map(|(n, name)| format!("#[path = \"{}\"]\npub mod _{};", name.to_str().unwrap(), n))
        .collect();
    fs::write(".leetcode.rs", segments.join("\n\n")).expect("unable to generate .leetcode.rs");
}

fn is_leetcode_file(entry: &DirEntry) -> bool {
    let re = Regex::new(r"^\d+\.\w+(\-\w+)+\.rs$").unwrap();
    return entry.file_type().unwrap().is_file()
        && re.is_match(entry.file_name().to_str().unwrap());
}
