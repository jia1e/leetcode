use regex::Regex;
use std::{
    env::current_dir,
    fs::{self, read_dir},
};

fn main() {
    let solutions_dir = "solutions";
    println!("cargo:rerun-if-changed={solutions_dir}");
    let mut files = read_dir(current_dir().unwrap().join(solutions_dir))
        .unwrap()
        .filter_map(|entry| match entry {
            Ok(entry) => {
                if entry.file_type().unwrap().is_file() {
                    let re = Regex::new(r"^(\d+)\.(\w+(\-\w+)*)\.rs$").unwrap();
                    let name = entry.file_name();
                    if let Some(captures) = re.captures(name.to_str().unwrap()) {
                        match (captures.get(0), captures.get(1), captures.get(2)) {
                            (Some(path), Some(n), Some(title)) => Some((
                                path.as_str().to_string(),
                                n.as_str().parse().unwrap(),
                                title.as_str().to_string(),
                            )),
                            _ => None,
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect::<Vec<(_, i32, _)>>();
    files.sort_by(|(_, a, _), (_, b, _)| a.cmp(b));
    let segments: Vec<_> = files
        .iter()
        .map(|(path, n, title)| {
            format!(
                r###"/// [{title}](https://leetcode.com/problems/{title})
#[path = "{solutions_dir}/{path}"]
pub mod p{n};
"###
            )
        })
        .collect();
    fs::write(
        "leetcode.rs",
        format!("mod shared;\n\n{}", segments.join("\n")),
    )
    .expect("unable to generate leetcode.rs");
}
