use std::fs::File;
use std::fmt;
use std::io::prelude::*;

#[derive(Debug)]
struct Entry {
    weight: f32,
    path: String
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
            Ok(w) =>  Ok(Entry{weight:w, path:parts[1].to_string()})
        }
    }
}

fn parse_file_contents(buf:&str) -> Vec<Entry> {
    buf.lines()
        .filter_map(|l| l.parse::<Entry>().ok())
        .collect()
}

fn parse_file(fname:&str) -> std::io::Result<Vec<Entry>> {
    let mut aj_file = File::open(fname)?;
    let mut contents = String::new();
    aj_file.read_to_string(&mut contents)?;
    Ok(parse_file_contents(contents.as_str()))
}

fn main()  -> std::io::Result<()>{
    let aj_fname: & 'static str = "/home/joel/.local/share/autojump/autojump.txt";
    let entries = parse_file(aj_fname)?;
    for e in &entries {
        println!("{}", e);
    }
    println!("#3: {}", entries[3].weight);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_test_file() {
        let entries = parse_file("tests/fine.txt");
        assert!(!entries.is_err());
        assert_eq!(3, entries.unwrap().len());
    }

    #[test]
    fn check_errors() {
        let entries = parse_file("tests/nan.txt");
        assert!(!entries.is_err());
        assert_eq!(0, entries.unwrap().len());
    }
}
