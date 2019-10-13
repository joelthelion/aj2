use std::fs::File;
use std::fmt;
use std::io::prelude::*;

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

fn main()  -> std::io::Result<()>{
    let mut aj_file = File::open("/home/joel/.local/share/autojump/autojump.txt")?;
    let mut contents = String::new();
    aj_file.read_to_string(&mut contents)?;
    let entries = parse_file_contents(contents.as_str());
    for e in &entries {
        println!("{}", e);
    }
    println!("#3: {}", entries[3].weight);
    Ok(())
}
