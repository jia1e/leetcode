use regex::Regex;
use std::{
    env::current_dir,
    fs::{self, read_dir},
    io::{BufRead, BufReader},
};

fn main() {
    let solutions_dir = "solutions";

    println!("cargo:rerun-if-changed={solutions_dir}");

    let mut files = read_dir(current_dir().unwrap().join(solutions_dir))
        .unwrap()
        .filter_map(|entry| {
            entry.map_or(None, |entry| {
                if entry.file_type().unwrap().is_file() {
                    match fs::File::open(entry.path()) {
                        Ok(file) => {
                            let mut buffer = BufReader::new(file);
                            let mut first_line = String::new();
                            let _ = buffer.read_line(&mut first_line);
                            if Regex::new(r"^\s*//\s*@ignore")
                                .unwrap()
                                .is_match(&first_line.as_str())
                            {
                                return None;
                            }
                            let re = Regex::new(r"^(\d+)\.(\w+(\-\w+)*)\.rs$").unwrap();
                            re.captures(entry.file_name().to_str().unwrap()).map_or(
                                None,
                                |captures| match (captures.get(0), captures.get(1), captures.get(2))
                                {
                                    (Some(path), Some(n), Some(title)) => Some((
                                        path.as_str().to_string(),
                                        n.as_str().parse().unwrap(),
                                        title.as_str().to_string(),
                                        Regex::new(r"^(\d+)-(.+)")
                                            .unwrap()
                                            .replace(title.as_str(), "$1$2")
                                            .to_string(),
                                    )),
                                    _ => None,
                                },
                            )
                        }
                        Err(_) => None,
                    }
                } else {
                    None
                }
            })
        })
        .collect::<Vec<(_, i32, _, _)>>();
    files.sort_by(|(_, a, _, _), (_, b, _, _)| a.cmp(b));

    let mut code = "mod shared;\n".to_string();

    files.iter().for_each(|(path, n, title, endpoint)| {
        code.push_str(
            format!(
                r###"
/// [{title}](https://leetcode.com/problems/{endpoint})
#[path = "{solutions_dir}/{path}"]
pub mod p{n};
"###
            )
            .as_str(),
        );
    });

    fs::write("leetcode.rs", code).expect("unable to generate leetcode.rs");
}
