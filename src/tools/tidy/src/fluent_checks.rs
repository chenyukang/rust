//! Checks that all Flunt files have messages in alphabetical order
//! Checks that all messages are referenced in the code

use crate::walk::{filter_dirs, walk};
use std::{fs::OpenOptions, io::Write, path::Path};

use regex::Regex;

lazy_static::lazy_static! {
    static ref MESSAGE: Regex = Regex::new(r#"(?m)^([a-zA-Z0-9_]+)\s*=\s*"#).unwrap();
}

fn filter_fluent(path: &Path) -> bool {
    if let Some(ext) = path.extension() { ext.to_str() != Some("ftl") } else { true }
}

fn filter_not_rust(path: &Path) -> bool {
    path.extension() != Some(std::ffi::OsStr::new("rs")) && !path.is_dir()
}

fn check_alphabetic(filename: &str, fluent: &str, bad: &mut bool) {
    let mut matches = MESSAGE.captures_iter(fluent).peekable();
    while let Some(m) = matches.next() {
        if let Some(next) = matches.peek() {
            let name = m.get(1).unwrap();
            let next = next.get(1).unwrap();
            if name.as_str() > next.as_str() {
                tidy_error!(
                    bad,
                    "{filename}: message `{}` appears before `{}`, but is alphabetically later than it
run `./x.py test tidy --bless` to sort the file correctly",
                    name.as_str(),
                    next.as_str()
                );
            }
        } else {
            break;
        }
    }
}

fn check_unused_messages(
    filename: &str,
    fluent: &str,
    bad: &mut bool,
    report: bool,
) -> Vec<String> {
    let mut matches = MESSAGE.captures_iter(fluent).peekable();
    let mut sources: Vec<String> = vec![];
    if matches.peek().is_some() {
        let src_dir = Path::new(filename).parent().unwrap().join("src");
        walk(
            &src_dir,
            |path, is_dir| filter_dirs(path) || (!is_dir && filter_not_rust(path)),
            &mut |_ent, contents| {
                sources.push(contents.to_string());
            },
        );
    }
    let mut unused_messages = vec![];
    while let Some(m) = matches.next() {
        let name = m.get(1).unwrap();
        // message maybe referenced as {name} in fluent files
        let name_var = "{".to_string() + name.as_str() + "}";
        if !sources.iter().any(|file| file.contains(name.as_str())) && !fluent.contains(&name_var) {
            unused_messages.push(name.as_str().to_owned());
            if report {
                tidy_error!(
                    bad,
                    "{filename}: message `{}` is not referenced in the code",
                    name.as_str()
                );
            }
        }
    }
    unused_messages
}

fn sort_messages(filename: &str, fluent: &str) -> String {
    let mut chunks = vec![];
    let mut cur = String::new();
    let unused_messages = check_unused_messages(filename, fluent, &mut false, false);
    for line in fluent.lines() {
        if MESSAGE.is_match(line) {
            let chunk = std::mem::take(&mut cur);
            if !unused_messages.iter().any(|key| chunk.starts_with(key)) {
                chunks.push(chunk);
            }
        }
        cur += line;
        cur.push('\n');
    }

    chunks.push(cur);
    chunks.sort();
    let mut out = chunks.join("");
    out = out.trim().to_string();
    out.push('\n');
    out
}

pub fn check(path: &Path, bless: bool, bad: &mut bool) {
    walk(
        path,
        |path, is_dir| filter_dirs(path) || (!is_dir && filter_fluent(path)),
        &mut |ent, contents| {
            let filename = ent.path().to_str().unwrap();
            if bless {
                let sorted = sort_messages(filename, contents);
                if sorted != contents {
                    let mut f =
                        OpenOptions::new().write(true).truncate(true).open(ent.path()).unwrap();
                    f.write(sorted.as_bytes()).unwrap();
                }
            } else {
                check_alphabetic(filename, contents, bad);
                check_unused_messages(filename, contents, bad, true);
            }
        },
    );
}
