use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
// use std::env;
use tempfile::NamedTempFile;

#[derive(Debug)]
struct Entry {
    weight: f32,
    path: String,
}

type Entries<'a> = HashMap<String, f32>;
fn dump_entries(entries: &Entries) -> String {
    let mut output = String::new();
    for (path, weight) in entries {
        output.push_str(format!("{}\t{}\n", weight, path).as_str());
    }
    output
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}", self.weight, self.path)
    }
}

impl std::str::FromStr for Entry {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.splitn(2, "\t").collect::<Vec<&str>>();
        let weight = parts[0].parse::<f32>();
        match weight {
            Err(_) => Err(()),
            Ok(w) => Ok(Entry {
                weight: w,
                path: parts[1].to_string(),
            }),
        }
    }
}

fn parse_file_contents(buf: &str) -> Entries {
    buf.lines()
        .filter_map(|l| {
            let entry = l.parse::<Entry>();
            match entry {
                Err(_) => None,
                Ok(e) => Some((e.path, e.weight)),
            }
        })
        .collect()
}

fn parse_file(fname: &str) -> std::io::Result<Entries> {
    let mut aj_file = File::open(fname)?;
    let mut contents = String::new();
    aj_file.read_to_string(&mut contents)?;
    Ok(parse_file_contents(contents.as_str()))
}

#[allow(dead_code)]
fn increase_weight(entries: &mut Entries, path: &str) {
    if let Some(weight) = entries.get_mut(path) {
        *weight += 1f32;
    } else {
        entries.insert(path.to_owned(), 1f32);
    }
}

fn main() -> std::io::Result<()> {
    // let args: Vec<String> = env::args().collect();
    const AJ_DIR: &'static str = "/home/joel/.local/share/autojump/";
    // let aj_fname: & 'static str = "/home/joel/.local/share/autojump/autojump.txt";
    let aj_fname = AJ_DIR.to_owned() + "autojump.txt";
    let entries = parse_file(aj_fname.as_str())?;
    let mut aj_tmp = NamedTempFile::new_in(AJ_DIR)?;
    aj_tmp.write_all(dump_entries(&entries).as_bytes())?;
    aj_tmp.persist(aj_fname)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_test_file() {
        let entries = parse_file("tests/fine.txt");
        assert!(!entries.is_err());
        let entries = entries.unwrap();
        assert_eq!(3, entries.len());
        assert_eq!(Some(&5f32), entries.get("titi"));
    }

    #[test]
    fn check_errors() {
        let entries = parse_file("tests/nan.txt");
        assert!(!entries.is_err());
        assert_eq!(0, entries.unwrap().len());
    }
}
